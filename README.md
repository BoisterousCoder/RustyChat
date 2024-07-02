# Yakking Yak
Yakking Yak is an end to end encrypted group chat app built with Rust. 

## What is it:

It is a group chat app that uses elliptic-curve diffie hellman to set up an AES-GCM 256 secure channel between devices. 

By default, other users are untrusted, and each message is sent encrypted with a specific target user’s shared key. 

## How do you use it:

This app depends on the Rust programming language, and Awaita for the GUI.

It uses port 4000 to communicate with the Firebase server, so it will have to be opened.
Note: The firebase server is just to setup the peer to peer connections and doesn't see any real data

## Required Libraries for building outside of cargo
- [Libadwaita](https://gtk-rs.org/gtk4-rs/stable/latest/book/libadwaita.html#libadwaita)
- [OpenSSL](https://www.openssl.org/)

On debian based distros like ubuntu do 
- `$ sudo apt install libadwaita-1-dev` for Libadwaita
- `$ sudo apt install libssl-dev` for OpenSSL

### To build:
  
Clone this repository
1. Setup a Firebase account if you dont already have one
1. Setup a [Firebase project with a Web App](https://firebase.google.com/docs/web/setup?authuser=0&hl=en) and get the associated API key
1. Setup a [Firebase real time database](https://firebase.google.com/docs/database/rtdb-vs-firestore) in the same project and get the database's url.
    -It should look something like `https://yourapp.firebaseio.com/`
1. Set the envoment variable `YAK_DB_KEY` to the API key you got earlier
1. Set the envoment variable `YAK_DB_URL` to the datababe url you got earlier
1. Run `$ cargo build`
1. Run `$ cargo run`

### How to Use
1. sign in by adding using your username and password or enter a new username and password to make a new account
    -  note: usernames are only unque locally
1. use the text box and the go button in the top right to change your chat room
    - note: for now you will have to re-establish trust when entering new chat rooms
1. when the checkbox in the bottom right is off use the text box and send button at the bottom to send unencrypted messages
1. to trust someone click on their name in the top right dropdown
1. once you have trusted one or more people and they have trusted you back you can send a message using the bottom text box with the checkbox turned on to send them an encrypted message

## Next steps
- Currently Messages are sent to a server to be sent to out to everyone. I plan on changing this to be peer to peer with the server just acting as an address swapping place. Maybe even a system to select the server you want to meet on.
- Add passwords to groups?
- Fix the randomness of the constants generated at the beggining
- Generate a new shared key every so often for increased security
- Start signing messages before sending for verification purposes
- The box of messages should scroll to the bottom when a message is sent
- The box of messages should pagify itself when there is a lot of messages
- Currently you can only listen in on one group at a time. I want to make it so you can listen to multiple
- Currently the group name is what you use to join the group. I want to make a group code seperate from the name

## Troubleshooting
- You may need to create a bin folder in the public folder for the program to build into
- You can delete the save folder to delete your users for testing or security. Every user's data is saved as flat files in that folder

## Other notes
- Web client is gone I dont want to manage two clients at once. It increases the workload too much. I went with libawaita as I want the client to be lightweight, and web clients are anything but.
- When voice and video chat is added (probably in the decently far future) there is a significant bandwidth penatly to the way I am handling end to end encryption. The data is encrypted once for each recipt, meaning the bandwidth usage is multiplied by the number of recipients.