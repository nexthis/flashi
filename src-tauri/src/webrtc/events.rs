use std::sync::Arc;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::RTCDataChannel;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use crate::compiler::compile;


pub fn on_message(msg: DataChannelMessage, channel: &Arc<RTCDataChannel>) {
    let msg_str = String::from_utf8(msg.data.to_vec()).unwrap();
    println!(
        "Message from DataChannel {}: '{}'",
        channel.label().to_owned(),
        msg_str
    );
    compile(msg_str);
}

pub fn on_statu_change(s: RTCPeerConnectionState) {
    println!("Peer Connection State has changed: {}", s);

    if s == RTCPeerConnectionState::Failed {
        // Wait until PeerConnection has had no network activity for 30 seconds or another failure. It may be reconnected using an ICE Restart.
        // Use webrtc.PeerConnectionStateDisconnected if you are interested in detecting faster timeout.
        // Note that the PeerConnection may come back from PeerConnectionStateDisconnected.
        println!("Peer Connection has gone to failed exiting");
        //let _ = done_tx.try_send(());
    }
}
