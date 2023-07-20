extern crate chrono;

use crate::lib::store::Crypto;
use crate::lib::utils::{decodeBase64, Address, split_and_clean, log};
use std::str;
use std::convert::TryInto;
use base64;
use chrono::{NaiveDateTime, Local};
use serde::{Deserialize, Serialize};

const INSECURE_LABEL:&str = "i";
const SECURE_LABEL:&str = "s";
const JOIN_LABEL:&str = "j";
const LEAVE_LABEL:&str = "l";
const PUBLIC_KEY_LABEL:&str = "p";
const TRUST_LABEL:&str = "t";
const BLANK_LABEL:&str = "_";

#[derive(Clone, Serialize, Deserialize)]
pub enum MsgContent{
	InsecureText(String),
	SecureText(Vec<SecureMsgIdentifier>),
	Join(String),
	PublicKey(String),
	Leave(String),
	Trust(Address),
	Blank()
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SecureMsgIdentifier {
	pub ord:usize,
	pub address:Address,
	pub is_sender:bool
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerMsg{
	pub from:Address,
	pub content:MsgContent,
	pub time_stamp:i64
}

impl ServerMsg{
	pub fn new(from:&Address, content:MsgContent) -> ServerMsg{
		let time_stamp = Local::now().timestamp_millis();
		return ServerMsg{
			from: from.clone(),
			content,
			time_stamp
		}
	}
	pub fn from_server(txt:&str, state:&mut Crypto) -> Option<ServerMsg>{
		let segments: Vec<&str> = split_and_clean(txt, '*');
		if segments.len() < 4 {
			return None;
		}
		let addr_segments: Vec<&str> = split_and_clean(segments[0], '@');
		let content_data = decodeBase64(segments[2]);

		let name = decodeBase64(addr_segments[0]);
		let device_id = addr_segments[1].parse().unwrap();
		let from = Address::new(&name, device_id);

		let content = match segments[1] {
			INSECURE_LABEL => MsgContent::InsecureText(content_data),
			SECURE_LABEL => {
				log(&format!("Proccessing Secure Message:{}", content_data));
				if let Some(secure_msg) = state.decrypt(&from, content_data){
					MsgContent::SecureText(vec![secure_msg])
				}else{
					return None
				}
			},
			JOIN_LABEL => MsgContent::Join(content_data),
			PUBLIC_KEY_LABEL => {
				if from != state.get_address(){
					state.add_public_key(from.clone(), decode_to_public_key_bytes(content_data.clone()));
				}
				MsgContent::PublicKey(content_data)
			},
			TRUST_LABEL => MsgContent::Trust(Address::from_sendable(content_data)),
			LEAVE_LABEL => MsgContent::Leave(content_data),
			BLANK_LABEL => MsgContent::Blank(),
			&_ => MsgContent::Blank()
		};

		let msg = ServerMsg{
			from, 
			content,
			time_stamp: segments[3].parse::<i64>().expect("timestamp is invalid")
		};
		state.add_msg(msg.clone());

		return Some(msg)
	}
	pub fn to_string(&self, state:&Crypto) -> String{
		let (kind, body):(&str, String) = match &self.content {
			MsgContent::PublicKey(public_key) => (PUBLIC_KEY_LABEL, public_key.to_string()),
			MsgContent::SecureText(ids) => {
				let mut encrypted_text:String = "".to_string();
				for id in ids {
					if let Some(payload) = state.get_encrypted_msg(id) {
						encrypted_text += &format!("{}*{}*{};", 
							id.address.as_sendable(), 
							id.ord, 
							base64::encode(payload)
						);
					}
				}
				
				(SECURE_LABEL, encrypted_text.to_string())
			},
			MsgContent::InsecureText(txt) => (INSECURE_LABEL, txt.to_string()),
			MsgContent::Join(group) => (JOIN_LABEL, group.to_string()),
			MsgContent::Leave(group) => (LEAVE_LABEL, group.to_string()),
			MsgContent::Trust(addr) => (TRUST_LABEL, addr.as_sendable()),
			MsgContent::Blank() => (BLANK_LABEL, String::from("_"))
		};
		return format!("*{}*{}*{}*", self.from.as_sendable(), kind, base64::encode(body.as_bytes()))
	}
	pub fn display(&self, state:&Crypto) -> Option<String>{
		let msg_data = match &self.content {
			MsgContent::PublicKey(pub_key) => {
				if state.agent_from_pub_key(pub_key).is_some() {
					None
				}else if self.from != state.get_address(){
					Some(("is alllowing people to trust them".to_string(), PUBLIC_KEY_LABEL))
				}else{
					None
				}
			},
			MsgContent::SecureText(id) => {
				if let Some(payload) = state.get_encrypted_msg(id.first().unwrap()){
					Some((str::from_utf8(payload.as_slice()).expect("Invalid utf8 on decrypt").to_string(), SECURE_LABEL))
				}else{
					Some(("has sent a secure message but you cannot read it as you do not trust them".to_string(), SECURE_LABEL))
				}
			},
			MsgContent::InsecureText(txt) => Some((txt.to_string(), INSECURE_LABEL)),
			MsgContent::Join(_) => Some(("went online".to_string(), JOIN_LABEL)),
			MsgContent::Leave(_) => Some(("went offline".to_string(), LEAVE_LABEL)),
			MsgContent::Trust(addr) => {
				#[cfg(target_arch = "wasm32")]
				let res = {
					let relation = state.relation(&addr);
					Some((format!("is trusting <span class=\"{}\">{}</span>", relation, addr.name), TRUST_LABEL))
				};
				#[cfg(not(target_arch = "wasm32"))]
				let res = Some((format!("{} is trusting {}", self.from.name, addr.name), TRUST_LABEL));
				res
			},
			MsgContent::Blank() => Some(("Error Parsing Text".to_string(), BLANK_LABEL))
		};
		let native_time = NaiveDateTime::from_timestamp_millis(self.time_stamp).expect("Invalid Timestap for message!");
		let date_time = native_time.format("%Y-%m-%d %H:%M:%S");

		#[cfg(target_arch = "wasm32")]
		return match msg_data {
			Some((content, label)) => {
				let relation = state.relation(&self.from);
				Some(format!("<span class=\"{}\">({}) {}</span> {}", relation, label, self.from.name, content.replace("\r", "")))
			},
			None => None
		};
		#[cfg(not(target_arch = "wasm32"))]
		return match msg_data {
			Some((content, label)) => {
				let relation = state.relation(&self.from);
				Some(format!("({}) {} at {}\r{}\r{}", label, self.from.name, date_time, content.replace("\r", ""), relation))
			},
			None => None
		};
	}
	pub fn to_writable(self, state:&Crypto) -> String {
		self.to_string(state)
	}
}

fn decode_to_public_key_bytes(s:String) -> [u8; 32]{
	let data = base64::decode(s).unwrap();
	let slice = data.as_slice();
	return match slice.try_into() {
        Ok(bytes) => bytes,
        Err(_) => panic!("Expected a Vec of length {} but it was {}", 32, data.len()),
    };
}