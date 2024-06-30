use constants::APP_ID;
use ::gtk::{glib::ExitCode, prelude::*};
use adw::{Application, gio::ApplicationFlags};
use messaging::socket::send_text;

use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

mod client;
mod gui;
mod constants;
mod messaging;

use crate::gui::build::build_sign_in;
use crate::client::state::Crypto;

lazy_static! {
    static ref GROUP:Mutex<String> = {
        Mutex::new("".to_string())
    };
    static ref STATE:Mutex<Crypto> = {
        Mutex::new(Crypto::empty())
    };
}

fn main() -> ExitCode {
    //The first emit doesn't seem to run right. This is a bypass for this.
    send_text("TEST", "WARMUP");

    let mut flags = ApplicationFlags::default();
    flags.set(ApplicationFlags::NON_UNIQUE, true);

    let app = Application::builder()
        .application_id(APP_ID)
        .flags(flags)
        .build();
    
    app.connect_activate(build_sign_in);

    app.run()
}
