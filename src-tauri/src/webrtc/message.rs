use std::sync::Arc;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::RTCDataChannel;

pub fn on_message(msg: DataChannelMessage, channel: &Arc<RTCDataChannel>) {
    let msg_str = String::from_utf8(msg.data.to_vec()).unwrap();
    println!(
        "Message from DataChannel {}: '{}'",
        channel.label().to_owned(),
        msg_str
    );
}
