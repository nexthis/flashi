use std::sync::Arc;

use crate::compiler::compile_with_chanel;
use crate::webrtc::payload::Payload;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::RTCDataChannel;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;

pub fn on_message(msg: DataChannelMessage, channel: Arc<RTCDataChannel>) {
    let msg_str = String::from_utf8(msg.data.to_vec()).unwrap();
    //let peyload = Payload::new(msg_str);
    println!(
        "Message from DataChannel {}: '{}'",
        channel.label().to_owned(),
        msg_str
    );
    compile_with_chanel(msg_str, channel).unwrap();
}

pub fn on_status_change(s: RTCPeerConnectionState) {
    println!("Peer Connection State has changed: {}", s);
}
