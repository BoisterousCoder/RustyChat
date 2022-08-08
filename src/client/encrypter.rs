//use std::time::SystemTime;
//use base64;
use crate::utils::ConnectionData;
//use crate::store::{attemptFetchIdData, storeID, RawIdData};

const DEVICE_ID:i32 = 12345;

pub struct Crypto{
	pub connData: ConnectionData,
	deviceId: i32
}
impl Crypto{
	pub fn new(connData:ConnectionData) -> Crypto{
		return Crypto{
			connData: connData,
			deviceId: DEVICE_ID
		}
	}
	pub fn addr(&self) -> Address{
		Address::new(&self.connData.name, self.deviceId)
	}
}

#[derive(Clone, Debug)]
pub struct Address{
	pub name: String,
	pub deviceId: i32
}
impl Address{
	pub fn new(name:&str, deviceId:i32) -> Address{
		return Address{
			name: name.to_string(),
			deviceId: deviceId
		}
	}
	pub fn asSendable(&self) -> String{
		return format!("{}@{}", base64::encode(self.name.clone()), self.deviceId)
	}
	// pub fn as_str(&self) -> &str{
	// 	return &self.name
	// }
}