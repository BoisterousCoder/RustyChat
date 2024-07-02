pub const MSG_CHECK_INTERVAL:u64 = 1500;//In miliseconds. 
pub const IS_AUTO_SAVING:bool = true;
pub const POLL_INTERVAL:u64 = 30000;//In miliseconds

pub const MAX_FAILED_CHECKS:u8 = 2;
pub const CONNECTION_ATTEMPTS:u8 = 2;
pub static ICE_SERVERS:[&'static str; 2] = ["stun:stun.l.google.com:19302", "stun:stun.l.google.com:19302"];
pub static APP_ID: &'static str = "com.BoisterousCoder.YakkingYak";
pub static APP_TITLE: &'static str = "Yakking Yak";
pub static FIREBASE_DATA: FirebaseData = FirebaseData{
    domain: env!("YAK_DB_URL", "Please set the 'YAK_DB_URL' path variable to a valid url for a firebase realtime database Ex: 'https://yourdatabase.firebaseio.com/'"),
    api_key: env!("YAK_DB_KEY", "Please set the 'YAK_API_KEY' path variable to a valid api key for a firebase realtime database"),
    user_collection_name: "sdp_group"
};


pub struct FirebaseData {
    pub domain: &'static str,
    pub api_key: &'static str,
    pub user_collection_name: &'static str
}