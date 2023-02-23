use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, RwLock};
use std::thread;

use crate::compiler::compile_with_chanel;
use crate::webrtc::payload::{Payload, PayloadType};
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::RTCDataChannel;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;

pub fn on_message(
    msg: DataChannelMessage,
    channel: Arc<RTCDataChannel>,
    last_message: Arc<RwLock<String>>,
) {
    let msg_str = String::from_utf8(msg.data.to_vec()).unwrap();
    let payload = Payload::new(msg_str);

    println!(
        "Message from DataChannel {}: '{}'",
        channel.label().to_owned(),
        payload.body()
    );

    match payload.variant() {
        PayloadType::Code => {
            thread::spawn(move || {
                compile_with_chanel(payload.body(), channel, last_message.clone()).unwrap();
            });
            return ();
        }
        _ => {
            let mut message = last_message.write().unwrap();

            println!("answer -->>>>: {}", payload.body());
            *message = payload.body();
        }
    }
}

pub fn on_status_change(s: RTCPeerConnectionState) {
    println!("Peer Connection State has changed: {}", s);
}
