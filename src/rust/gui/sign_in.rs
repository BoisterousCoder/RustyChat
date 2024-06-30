
use std::{path::Path, fs::{self, File}, io::{Write, Read}};

use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;
use rand_core::{RngCore, OsRng};

use crate::client::utils::{calc_hash, log};

const SAVE_DIR:&str = "./saves";
const FILE_EXTENTION:&str = "user";

pub struct SignInDetails {
    pub private_device_id:[u8; 32],
    pub username:String,
    pub password:String
}

pub fn on_sign_in_attempt(username:&str, password:&str) -> Option<SignInDetails>{ 
    if !Path::new(SAVE_DIR).is_dir() {
        fs::create_dir(SAVE_DIR).expect("can't make save directory");
    }

    #[allow(deprecated)]
    let filename = format!("{}/{}.{}", SAVE_DIR, base64::encode(&username), FILE_EXTENTION);
    if !fs::metadata(&filename).is_ok() {
        log("User file is missing. Making a new user and file");
        let mut device_id = [0u8; 32];
        OsRng.fill_bytes(&mut device_id);
        #[allow(deprecated)]
        let device_id_str = base64::encode(device_id);

        let mut file = File::create(&filename).expect("unable to create file");
        let key = new_magic_crypt!(calc_hash(&password), 256);
        let data = key.encrypt_str_to_base64(device_id_str);
        
        file.write_all(data.as_bytes()).unwrap();
        file.sync_all().unwrap();

        return Some(SignInDetails {
            private_device_id: device_id, 
            username: username.to_string(), 
            password: password.to_string() 
        });
    }else{
        log("User file found!");
        let mut file = File::open(filename).expect("Unable to open user file");
        let mut data = String::new();
        if file.read_to_string(&mut data).is_ok(){
            let key = new_magic_crypt!(calc_hash(&password), 256);
            if let Some(device_id_str) = key.decrypt_base64_to_string(&data).ok(){
                #[allow(deprecated)]
                let device_id_vec = base64::decode(device_id_str).unwrap();
                let mut device_id = [0u8; 32];
                let mut i = 0;
                for byte in device_id_vec{
                    device_id[i] = byte;
                    i+=1;
                    if i == device_id.len(){
                        break;
                    }
                };

                return Some(SignInDetails { 
                    private_device_id: device_id, 
                    username: username.to_string(), 
                    password: password.to_string() 
                });
            };
        }
        return None;
    }
}