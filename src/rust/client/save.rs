use serde::{Serialize, Deserialize};

use crate::client::forein_agent::ForeinAgent;
use crate::client::serverhandlers::ServerMsg;
use crate::client::ratchet::Ratchet;
use crate::client::utils::Address;
use crate::client::utils::calc_hash;

use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;

use std::io::prelude::*;
use std::fs;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::thread;

const SAVE_DIR:&str = "./saves";
const FILE_EXT:&str = "msglog";

#[derive(Serialize, Deserialize, Clone)]
pub struct GroupSave {
    pub group:String,
    pub addr:Address,
    pub proxy_ratchet:Ratchet,
	pub agents: Vec<ForeinAgent>,
	pub msgs: Vec<ServerMsg>
}

impl GroupSave {
    pub fn save(self, password:&str) -> Result<(), Box<dyn Error>>{
        let plain_data = serde_json::to_string(&self).unwrap();
        let hashed_password = calc_hash(&password);

        thread::spawn(move || {
            if !Path::new(SAVE_DIR).is_dir() {
                fs::create_dir(SAVE_DIR).expect("Failed to create directory!");
            }
            let mut file = File::create(Self::filename(&self.addr, &self.group)).expect("Failed to get or create file");
            let key = new_magic_crypt!(hashed_password, 256);
            let data = key.encrypt_str_to_base64(plain_data);
    
            file.write_all(data.as_bytes()).expect("Failed to write to file");
            file.sync_all().expect("Failed to release file from control");
        });
        Ok(())
    }
    pub fn load(address:Address, group:&str, password:&str) -> Option<Self>{
        let filename = Self::filename(&address, group);
        let hashed_password = calc_hash(&password);

        if fs::metadata(&filename).is_ok(){
            if let Some(mut file) = File::open(filename).ok(){
                let mut data = String::new();
                if file.read_to_string(&mut data).is_ok(){
                    let key = new_magic_crypt!(hashed_password, 256);
                    if let Some(plain_data) = key.decrypt_base64_to_string(&data).ok(){
                        let is_group:Option<Self> = serde_json::from_str(&plain_data).ok();
                        if let Some(group) = is_group {
                            let mut mut_group = group.clone();
                            for agent in &mut mut_group.agents{
                                agent.is_online = false;
                            }
                            return Some(mut_group);
                        }
                    }
                }
            };
        }
        return None;
    }
    fn filename(addr:&Address, group:&str) -> String {
        #[allow(deprecated)]
        let safe_group = base64::encode(group);
        #[allow(deprecated)]
        let filename = &format!("{}@{}", base64::encode(addr.name()), base64::encode(safe_group));
        format!("{}/{}.{}", SAVE_DIR, filename, FILE_EXT)
    }
}