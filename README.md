# Yakking Yak
Yakking Yak is an end to end encrypted group chat app built with Rust and Node JS for the server. 

## What is it:

It is a group chat app that uses elliptic-curve diffie hellman to set up an AES-GCM 256 secure channel between devices. 

By default, other users are untrusted, and each message is sent encrypted with a specific target user’s shared key. 

## How do you use it:

This app depends on the Rust programming language, and node js.

It uses port 4000 to communicate with the server, so it will have to be opened locally.

## Required Libraries for building outside of npm and cargo
- [Libadwaita](https://gtk-rs.org/gtk4-rs/stable/latest/book/libadwaita.html#libadwaita)

On debian based distros like ubuntu do `$ sudo apt install libadwaita-1-dev`

### To build:
  
Clone this repository
1. Run `$ npm install` to install required nodejs libraries
1. Run `$ npm run build` to build the rust web assembly library to build the pug files to html
1. Run `$ npm run start` to start the node server   
1. Run `$ cargo run` to start the adwaita client

### How to Use
1. sign in by adding using your username and password or enter a new username and password to make a new account
    -  note: usernames are only unque locally
1. use the text box and the go button in the top right to change your chat room
    - note: for now you will have to re-establish trust when entering new chat rooms
1. when the checkbox in the bottom right is off use the text box and send button at the bottom to send unencrypted messages
1. to trust someone click on their name in the top right dropdown
1. once you have trusted one or more people and they have trusted you back you can send a message using the bottom text box with the checkbox turned on to send them an encrypted message

## Next steps
- Currently the chat log is ephemeral, I am developing a distributed chat log using CRDT to make a persistent log.
- Currently Messages are sent to a server to be sent to out to everyone. I plan on changing this to be peer to peer. 
- Currently the method of restoring saved data is broken. This needs to be fixed
- Currently encryption of data is handled by the cpu. One data I want to be able to encrypt video and voice data. Inorder to accomidate this I want to move the encryption to the gpu.


## Troubleshooting
You may need to create a bin folder in the public folder for the program to build into

## Other notes
web client is being depricated for the adwaita client