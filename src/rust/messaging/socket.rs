use std::sync::Arc;

use crossbeam_queue::SegQueue;
use rust_socketio::{client::Client, ClientBuilder, Payload};

use crate::{client::utils::log, constants::{MSG_TYPES, SOCKET_SERVER_ADDRESS}};

lazy_static! {
    pub static ref MSG_QUEUE:SegQueue::<String> = SegQueue::new();
    static ref SOCKET_CLIENT:Arc<Client> = {
        let on_msg = move |payload_wrapped, _| {
            if let Payload::String(payload) = payload_wrapped {
                log(&format!("Recieved Msg {}", &payload));
                let payload_fixed = payload.replace("\"", "");
                MSG_QUEUE.push(payload_fixed);
            };
        };
    
        let mut socket_builder = ClientBuilder::new(SOCKET_SERVER_ADDRESS).namespace("/");
        for msg_type in MSG_TYPES {
            socket_builder = socket_builder.on(msg_type.to_string(), on_msg.clone());
        }
    
        Arc::new(socket_builder.connect().expect("Unable to connect to server"))
    };
}

pub fn send_text(channel:&str, text:&str) {
    SOCKET_CLIENT.emit( channel, text).unwrap_or_else(move |_|{
        panic!("Failed to send message to channel: {}\nThe message that failed to send was\n{}", channel, text)
    });
}