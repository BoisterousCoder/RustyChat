use crate::client::state::Crypto;
use crate::messaging::socket::send_text;
use crate::{GROUP, STATE};

use crate::client::save::GroupSave;
use crate::client::utils::{log, Address};
use crate::client::serverhandlers::{MsgContent, ServerMsg};
use ::gtk::{Button, Box, CheckButton, Entry, prelude::*};
use rand_core::{OsRng, RngCore};

use crate::gui::sign_in::on_sign_in_attempt;

use super::build::build_content;


pub fn on_user_click(from:&Address){
    let state = &mut STATE.lock().unwrap();
    if state.relation(from) == "allowedTrust".to_string(){
        let content = match state.trust(from.name.to_string()) {
            Some(forein) => Some(MsgContent::Trust(forein.clone())),
            None => None
        };
        if content.is_some() {
            let msg = ServerMsg::new(&state.get_address(), content.unwrap());
            send_text("t",&msg.to_string(&state));
        }
    }else{
        log(&format!("Can't trust {} because you already trust them, you dont have their primary key, or it's you. Can't trust yourself after all.", from.name))
    }
}

pub fn on_join_group(group_entry:&Entry){
    let state = &mut STATE.lock().unwrap();

    let group = &mut GROUP.lock().unwrap();
    if !group.is_empty(){
        let old_save = state.group_as_save(&group);
        old_save.save(&state.password).expect("Unable to save the current group");
    }
    
    let new_group = group_entry.buffer().text().to_string();
    **group = new_group;
    
    if let Some(save) = GroupSave::load(state.get_address(), &group, &state.password){
        state.load_group_save(save);
        log("Successfully loaded group")
    }else{
        state.new_group(OsRng.next_u64(), OsRng.next_u64());
        log("Changed to new group");
    }

    let content = MsgContent::Join(group.to_string());
    let msg =  ServerMsg::new(&state.get_address(), content);
    send_text("j",&msg.to_string(&state));
    state.update_online_statuses();
}
pub fn on_send_msg(msg_entry:&Entry){
    let state = &mut STATE.lock().unwrap();
    let text = msg_entry.buffer().text().to_string();
    if text.is_empty(){
        return ();
    }
    let encyption_checkbox:CheckButton = msg_entry.next_sibling().unwrap()
        .next_sibling().unwrap()
        .downcast().expect("Found UI emlement but is not checkbutton! UI is broke pls fix");
    
    let (content, label) = if encyption_checkbox.is_active(){
        let encrypted_text = state.encrypt(text);
        (MsgContent::SecureText(encrypted_text), "s")
    }else {
        (MsgContent::InsecureText(text), "i")
    };
    let msg = ServerMsg::new(&state.get_address(), content);

    send_text(&label,&msg.to_string(&state));
    msg_entry.buffer().set_text("");
}

pub fn on_sign_in(sign_in_button:&Button){
    let username_input:Entry = sign_in_button
        .parent().unwrap()
        .first_child().unwrap()
        .next_sibling().unwrap()
        .downcast().expect("Found UI emlement but is not entry! UI is broke pls fix");
    let password_input:Entry = username_input
        .next_sibling().unwrap()
        .next_sibling().unwrap()
        .downcast().expect("Found UI emlement but is not entry! UI is broke pls fix");
    let username = username_input.buffer().text().to_string();
    let password = password_input.buffer().text().to_string();
    
    if let Some(options) = on_sign_in_attempt(&username, &password){
        sign_in_button.parent().unwrap().hide();
        let state = &mut STATE.lock().unwrap();
        **state = Crypto::new(
            &options.username, 
            &options.password, 
            options.private_device_id, 
            OsRng.next_u64(), 
            OsRng.next_u64());

        let content:Box = sign_in_button
            .parent().unwrap()
            .parent().unwrap()
            .downcast().expect("Found UI emlement but is not entry! UI is broke pls fix");
        build_content(&content);
    };
}