use std::sync::{Arc, Mutex};

use futures_executor::block_on;
use crossbeam_queue::SegQueue;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::api::{APIBuilder, API};

use crate::constants::{CONNECTION_ATTEMPTS, ICE_SERVERS};
use crate::client::utils::log;

lazy_static! {
    pub static ref MSG_QUEUE:SegQueue::<String> = SegQueue::new();
    pub static ref PEERS:Mutex<Peers> = Mutex::new(block_on(Peers::new()));
}

pub struct Peers {
    list:Vec<Peer>,
    api:Arc<API>,
    config:RTCConfiguration,
}
impl Peers {
    pub async fn new() -> Self{
        let config = RTCConfiguration{
            ice_servers: vec![RTCIceServer{
                urls:ICE_SERVERS.iter().map(|s| s.to_string()).collect(),
                ..Default::default()
            }],
            ..Default::default()
        };

        let mut media_engine = MediaEngine::default();
        media_engine.register_default_codecs().unwrap();
        let mut registry = Registry::new();
        registry = register_default_interceptors(registry, &mut media_engine).unwrap();
        let api = APIBuilder::new()
            .with_media_engine(media_engine)
            .with_interceptor_registry(registry)
            .build();

        let mut ret =  Self{
            api:Arc::new(api),
            list: vec![],
            config
        };
        ret.new_peer().await;
        return ret;
        
    }

    async fn new_peer(&mut self){
        let peer = Peer::new(&self.api, self.config.clone()).await; 
        self.list.push(peer);
    }
    pub async fn send_text(&self, text:&str){
        for peer in self.list.iter(){
            peer.send_text(text).await;
        }
    }
    pub async fn connect_to(&mut self, sdp:&str, public_key:&str){
        if let Some(empty_conn) = &mut self.list.last_mut(){
            empty_conn.connect_to(sdp, public_key).await;
        }
        self.new_peer().await;
    }
    pub fn get_sdp(&self) -> String{
        return self.list.last().unwrap().sdp.to_string();
    }
    pub async fn close_and_empty(&mut self){
        for peer in &self.list{
            peer.connection.close().await.unwrap();
        }
        self.list = vec![];
        self.new_peer().await;
    }
}
struct Peer{
    sdp:String,
    connection:Arc<RTCPeerConnection>,
    data_channel:Arc<RTCDataChannel>,
    public_key:Option<String>
}
impl Peer {
    pub async fn new(api:&Arc<API>, config:RTCConfiguration) -> Self
    {
        let connection: Arc<RTCPeerConnection> = Arc::new(api.new_peer_connection(config.clone()).await.unwrap());

        //create a data connection to be able send message to the peer
        let data_channel = connection.create_data_channel("data", None).await.unwrap();
        
        //start liseners
        connection.on_peer_connection_state_change(Box::new(move |connection_state: RTCPeerConnectionState| {
            log(&format!("Peer Connection State has changed: {connection_state}"));
            Box::pin(async {})
        }));
        data_channel.on_open(Box::new(move || {
            println!("Data channel successfully opened for a peer");
            Box::pin(async {})
        }));
        data_channel.on_message(Box::new(move |msg: DataChannelMessage| {
            let msg_str = String::from_utf8(msg.data.to_vec()).unwrap();
            println!("Message on DataChannel '{msg_str}'");
            MSG_QUEUE.push(msg_str);
            Box::pin(async {})
        }));

        // Create an offer to send to the browser
        let offer = connection.create_offer(None).await.unwrap();

        // Create channel that is blocked until ICE Gathering is complete
        let mut gather_complete = connection.gathering_complete_promise().await;
        let sdp = (&offer.sdp).clone();

        // Sets the LocalDescription, and starts our UDP listeners
        connection.set_local_description(offer).await.unwrap();

        // Block until ICE Gathering is complete, disabling trickle ICE
        // we do this because we only can exchange one signaling message
        // in a production application you should exchange ICE Candidates via OnICECandidate
        let _ = gather_complete.recv().await;

        return Self {
            sdp,
            connection,
            data_channel,
            public_key:None
        }
    }
    pub async fn send_text(&self, text:&str){
        if self.data_channel.negotiated(){
            self.data_channel.send_text(text).await.unwrap();
        }else{
            log("no one to send message to")
        }
    }
    pub async fn connect_to(&mut self, sdp: &str, public_key:&str) -> bool{
        self.public_key = Some(public_key.to_string());

        //this is where we would put in the data
        let answer = RTCSessionDescription::answer(sdp.to_string()).unwrap();

        // Apply the answer as the remote description
        let mut trys = 0u8;
        loop{
            if trys >= CONNECTION_ATTEMPTS{
                return false;
            }else if self.connection.set_remote_description(answer.clone()).await.is_ok(){
                return true;
            }else{
                trys += 1;
            }
        }
    }
}

// fn open_data_channel_recursive(previous_data_channel: &Arc<RTCDataChannel>){
//     let data_channel = Arc::clone(previous_data_channel);
//     data_channel.on_open(Box::new(move || {
//         println!("Data channel '{}'-'{}' open. Random messages will now be sent to any connected DataChannels every 5 seconds", d1.label(), d1.id());

//         let d2 = Arc::clone(&d1);
//         Box::pin(async {})
//     }));
// }