use std::{collections::HashMap, sync::Mutex};

use firebase_rs::Firebase;
use serde::{Deserialize, Serialize};

use crate::constants::FIREBASE_DATA;


lazy_static!{
    pub static ref SEVER:Mutex<Server> = Mutex::new(Server::new());
}

pub struct Server{
    firebase:Firebase,
    firebase_id:Option<String>,
    last_group:Option<String>
}
impl Server{
    pub fn new() -> Self{
        let firebase = Firebase::auth(&FIREBASE_DATA.domain, FIREBASE_DATA.api_key).unwrap();
        Self{
            firebase,
            firebase_id:None,
            last_group:None
        }
    }
    pub async fn register_to_group(&mut self, public_key:&str, sdp:&str, group:&str){
        //If we changed groups or is our first time
        if !self.last_group.as_ref().is_some_and(|old_group| old_group == group){
            //If it isn't our first time
            if let Some(firebase_id) = &self.firebase_id{
                self.firebase
                    .at(&FIREBASE_DATA.user_collection_name)
                    .at(self.last_group.as_ref().unwrap())
                    .at(&firebase_id)
                    .delete().await.unwrap();
            }

            let res = self.firebase
                .at(FIREBASE_DATA.user_collection_name)
                .at(group)
                .set(&PeerConnectionData{
                    public_key:public_key.to_string(), 
                    sdp:sdp.to_string()
            }).await.unwrap().data;
            let firebase_id = serde_json::from_str::<FirebaseId>(&res).unwrap().name;
            self.firebase_id = Some(firebase_id);
            self.last_group = Some(group.to_string());
        }else{
            if let Some(firebase_id) = &self.firebase_id{
                self.firebase
                    .at(&FIREBASE_DATA.user_collection_name)
                    .at(group)
                    .at(firebase_id)
                    .update(&PeerConnectionData{
                        public_key:public_key.to_string(), 
                        sdp:sdp.to_string()
                }).await.unwrap();
            }else {
                panic!("reached impossible branch")
            }
        }
    }
    pub async fn get_peers_data_in_group(&self)->Vec<PeerConnectionData>{
        if let Some(group) = &self.last_group {
            let res:HashMap<String, PeerConnectionData>= self.firebase
                .at(&FIREBASE_DATA.user_collection_name)
                .at(group).get()
            .await.unwrap();
            let conn_data:Vec<PeerConnectionData> = res.into_iter().map(|(_id, data)| data).collect();
            return conn_data;
        }else {
            panic!("One must register to a group before getting the sdps and public keys of peers in that group")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PeerConnectionData {
    pub public_key: String,
    pub sdp: String
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct FirebaseId {
    pub name: String
}