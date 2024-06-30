use ::gtk::{glib::ExitCode, prelude::*};
use adw::{Application, gio::ApplicationFlags};

use rand_core::{OsRng, RngCore};
use std::sync::{Mutex, Arc};
use rust_socketio::client::Client;
use rust_socketio::{ClientBuilder, Payload};
use crossbeam_queue::SegQueue;

#[macro_use]
extern crate lazy_static;

mod client;
mod gui;

use crate::gui::build::build_sign_in;
use crate::client::state::Crypto;
use crate::client::utils::log;

static APP_ID: &str = "com.BoisterousCoder.YakkingYak";
static APP_TITLE: &str = "Yakking Yak";

const MSG_CHECK_INTERVAL:u64 = 1500;//In miliseconds. This is how long it 
const SEED:u64 = 1234567890; //TODO: fix the seed to its actually random
const PASSWORD:&str = "ABCDE";
const PROXY_SEED:u64 = 0987654321; //TODO: fix the seed to its actually random
const DEVICE_ID:[u8; 32] = [1u8; 32];//TODO: Make this useful
const MSG_TYPES:[char; 6] = ['i', 's', 't', 'l', 'p', 'j'];
const SOCKET_SERVER_ADDRESS:&'static str = "http://localhost:4000";
const IS_AUTO_SAVING:bool = true;
const POLL_INTERVAL:u64 = 30000;//In miliseconds
const MAX_FAILED_CHECKS:u8 = 2;

lazy_static! {
    static ref GROUP:Mutex<String> = {
        Mutex::new("".to_string())
    };
    static ref STATE:Mutex<Crypto> = {
        let user_number:u32 = OsRng.next_u32();
        let user_name = format!("Anon{:X}", user_number);
        Mutex::new(Crypto::new(&user_name, PASSWORD, DEVICE_ID, OsRng.next_u64(), OsRng.next_u64()))
    };
    pub static ref MSG_QUEUE:SegQueue::<String> = SegQueue::new();
    pub static ref SOCKET_CLIENT:Arc<Client> = {
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

fn main() -> ExitCode {
    //The first emit doesn't seem to run right. This is a bypass for this.
    SOCKET_CLIENT.emit("TEST", "WARMUP").unwrap();

    let mut flags = ApplicationFlags::default();
    flags.set(ApplicationFlags::NON_UNIQUE, true);

    let app = Application::builder()
        .application_id(APP_ID)
        .flags(flags)
        .build();
    
    app.connect_activate(build_sign_in);

    app.run()
}
