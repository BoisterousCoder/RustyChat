use ::gtk::{prelude::*, Button};
use ::gtk::{Box, ListBox, Orientation, Popover};

use crate::constants::{IS_AUTO_SAVING, MSG_CHECK_INTERVAL, POLL_INTERVAL};
use crate::{GROUP, MSG_QUEUE, SOCKET_CLIENT, STATE};
use crate::gui::build::display_msg;
use crate::client::serverhandlers::{MsgContent, ServerMsg};
use crate::client::state::Crypto;
use crate::gui::events::on_user_click;
use crate::client::utils::log;

pub fn do_ui_loop(iterations_since_last_poll:&u64, msg_list:&ListBox, user_list:&Popover) -> bool{
    let state = &mut STATE.lock().expect("unable to aquire state");
    while let Some(txt) = MSG_QUEUE.pop() {
        log("handing msg");

        if let Some(msg) = ServerMsg::from_server(&txt, state){
            update_msg_display(msg_list, user_list, state);

            if let MsgContent::Join(_) = msg.content {
                send_public_key(&state)
            }
        }
    };
    if iterations_since_last_poll * MSG_CHECK_INTERVAL >= POLL_INTERVAL {
        send_public_key(state);
        state.update_online_statuses();
        update_msg_display(msg_list, user_list, state);
        return true;
    }
    return false;
}

fn send_public_key(state: &Crypto){
    let content_to_send = MsgContent::PublicKey(state.public_key());
    let msg_to_send = ServerMsg::new(&state.get_address(), content_to_send);
    SOCKET_CLIENT.emit("p", msg_to_send.to_string(&state)).expect("unable to send primary keys");
}

pub fn update_msg_display(msg_list:&ListBox, user_list:&Popover, state:&Crypto){
    while let Some(child) = msg_list.first_child() {
        msg_list.remove(&child);
    }
    for msg in state.get_msgs(){
        display_msg(msg_list, msg, state);
    }
    
    let user_list_box = Box::new(Orientation::Vertical, 5);
    for agent in state.get_agents(){
        if agent.is_online{
            let relation = state.relation(&agent.keys.address);
            let relation_display = if &relation == "self" || &relation == "trusted" {
                relation
            }else{
                "untrusted".to_string()
            };
            let user_display = format!("{}--{}",
                relation_display,
                agent.keys.address.name);
            let user_button = Button::builder()
                .label(user_display)
                .halign(::gtk::Align::Start)
                .hexpand(true)
                .build();
            user_button.connect_clicked(move |_| on_user_click(&agent.keys.address));
            user_list_box.append(&user_button);
        }
    }
    user_list.set_child(Some(&user_list_box));
    
    if IS_AUTO_SAVING{
        state.group_as_save(&GROUP.lock().unwrap())
            .save(&state.password)
            .expect("Failed to autosave!");
    }
}