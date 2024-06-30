pub const MSG_CHECK_INTERVAL:u64 = 1500;//In miliseconds. This is how long it 
pub const SEED:u64 = 1234567890; //TODO: fix the seed to its actually random
pub const PASSWORD:&str = "ABCDE";
pub const PROXY_SEED:u64 = 0987654321; //TODO: fix the seed to its actually random
pub const DEVICE_ID:[u8; 32] = [1u8; 32];//TODO: Make this useful
pub const MSG_TYPES:[char; 6] = ['i', 's', 't', 'l', 'p', 'j'];
pub const SOCKET_SERVER_ADDRESS:&'static str = "http://localhost:4000";
pub const IS_AUTO_SAVING:bool = true;
pub const POLL_INTERVAL:u64 = 30000;//In miliseconds
pub const MAX_FAILED_CHECKS:u8 = 2;

pub static APP_ID: &str = "com.BoisterousCoder.YakkingYak";
pub static APP_TITLE: &str = "Yakking Yak";
