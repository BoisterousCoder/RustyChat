use serde::{Serialize, Deserialize};
use crate::client::key_bundle::KeyBundle;
use crate::client::ratchet::Ratchet;
use crate::constants::MAX_FAILED_CHECKS;

#[derive(Serialize, Deserialize, Clone)]
pub struct ForeinAgent {
    pub to_ratchet:Option<Ratchet>,
    pub from_ratchet:Option<Ratchet>,
    pub keys:KeyBundle,
    pub is_online:bool,
    pub failed_checks:u8,
    pub recently_sent_poll:bool
}
impl ForeinAgent{
    pub fn update_online_status(&mut self){
        if self.recently_sent_poll {
            self.is_online=true;
            self.failed_checks = 0;
            self.recently_sent_poll = false;
        }else if self.is_online {
            self.failed_checks += 1;
            if self.failed_checks >= MAX_FAILED_CHECKS {
                self.is_online = false;
            }
        }else if self.failed_checks != 0 {
            self.failed_checks = 0;
        }
    }
}
