//use crate::state::GlobalState;
use serde_json::Value;
use std::sync::Arc;
use tauri::async_runtime::block_on;
use tauri::Window;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_candidate::{RTCIceCandidate, RTCIceCandidateInit};
use webrtc::ice_transport::ice_connection_state::RTCIceConnectionState;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

mod events;

//TODO: This is proof of concept, try to refactor
#[tauri::command]
pub async fn connect(offer: String, window: Window) -> Result<RTCSessionDescription, String> {
    // Create a MediaEngine object to configure the supported codec
    let mut m = MediaEngine::default();

    let window = Arc::new(window);

    // Register default codecs
    match m.register_default_codecs() {
        Ok(_) => (),
        Err(error) => return Err(error.to_string()),
    }

    // Create a InterceptorRegistry. This is the user configurable RTP/RTCP Pipeline.
    // This provides NACKs, RTCP Reports and other features. If you use `webrtc.NewPeerConnection`
    // this is enabled by default. If you are manually managing You MUST create a InterceptorRegistry
    // for each PeerConnection.
    let mut registry = Registry::new();

    // Use the default set of Interceptors
    registry = match register_default_interceptors(registry, &mut m) {
        Ok(val) => val,
        Err(error) => return Err(error.to_string()),
    };

    // Create the API object with the MediaEngine
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    // Prepare the configuration
    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    // Create a new RTCPeerConnection
    let peer_connection = Arc::new(match api.new_peer_connection(config).await {
        Ok(val) => val,
        Err(error) => return Err(error.to_string()),
    });

    let window_ref = Arc::clone(&window);
    let peer_connection_ref = Arc::clone(&peer_connection);

    let window_event_id = window_ref.listen("remote-ice-candidate", move |event| {
        let test = event.payload().unwrap();
        let value: Value = serde_json::from_str(test).unwrap();

        println!("text: {}", test);

        println!(
            "got window remote-ice-candidate with payload {}, {}, {}",
            value["candidate"], value["sdpMLineIndex"], value["sdpMid"]
        );

        let peer_connection_ref = Arc::clone(&peer_connection_ref);

        tauri::async_runtime::spawn(async move {
            let result = peer_connection_ref
                .add_ice_candidate(RTCIceCandidateInit {
                    candidate: value["candidate"].to_string(),
                    sdp_mid: Some(value["sdpMid"].to_string()),
                    sdp_mline_index: value["sdpMLineIndex"].to_string().parse().ok(),
                    username_fragment: Some("def".to_string().to_string()),
                })
                .await
                .ok();
            println!("remote-ice-candidate added: {:?}", result);

            result
        });
    });

    peer_connection
        .on_ice_connection_state_change(Box::new(move |state| {
            println!("on_ice_connection_state_change: {}", state);
            if RTCIceConnectionState::Checking != state {
                window_ref.unlisten(window_event_id);
                println!("unlisten(window_event_id)");
            }
            Box::pin(async move {})
        }))
        .await;

    let window_ref = Arc::clone(&window);

    peer_connection
        .on_ice_candidate(Box::new(move |ice_candidate: Option<RTCIceCandidate>| {
            if ice_candidate.is_none() {
                return Box::pin(async {});
            }

            let ice_candidate = ice_candidate.unwrap();
            // let value = ice_candidate.;

            //ice_candidate
            println!(
                "ICE Connection State has changed: {}",
                ice_candidate.to_string()
            );

            let window_ref = Arc::clone(&window_ref);
            Box::pin(async move {
                window_ref
                    .emit("on-ice-candidate", ice_candidate.to_json().await.unwrap())
                    .unwrap();
            })
        }))
        .await;

    let window_ref = Arc::clone(&window);
    // Set the handler for Peer connection state
    // This will notify you when the peer has connected/disconnected
    peer_connection
        .on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
            //Dispach event
            window_ref
                .emit("peer-connection-state-change", s.to_string().to_lowercase())
                .unwrap();

            //Run event
            events::on_status_change(s);

            Box::pin(async {})
        }))
        .await;

    // Register data channel creation handling
    peer_connection
        .on_data_channel(Box::new(move |d: Arc<RTCDataChannel>| {
            let d_label = d.label().to_owned();
            let d_id = d.id();
            println!("New DataChannel {} {}", d_label, d_id);

            // Register channel opening handling
            Box::pin(async move {
                let d2 = Arc::clone(&d);

                // Register text message handling
                d.on_message(Box::new(move |msg: DataChannelMessage| {
                    //Run event
                    events::on_message(msg, &d2);
                    Box::pin(async {})
                }))
                .await;
            })
        }))
        .await;

    let offer = match serde_json::from_str::<RTCSessionDescription>(offer.as_str()) {
        Ok(val) => val,
        Err(error) => {
            println!("{}, {}", error, offer);
            return Err(error.to_string());
        }
    };

    // Set the remote SessionDescription
    match peer_connection.set_remote_description(offer).await {
        Ok(val) => val,
        Err(error) => return Err(error.to_string()),
    };

    // Create an answer
    let answer = match peer_connection.create_answer(None).await {
        Ok(val) => val,
        Err(error) => return Err(error.to_string()),
    };

    // Create channel that is blocked until ICE Gathering is complete
    // let mut gather_complete = peer_connection.gathering_complete_promise().await;

    // Sets the LocalDescription, and starts our UDP listeners
    match peer_connection.set_local_description(answer).await {
        Ok(val) => val,
        Err(error) => return Err(error.to_string()),
    };

    // Block until ICE Gathering is complete, disabling trickle ICE
    // we do this because we only can exchange one signaling message
    // in a production application you should exchange ICE Candidates via OnICECandidate
    //let _ = gather_complete.recv().await;

    // Output the answer in base64 so we can paste it in browser
    if let Some(local_desc) = peer_connection.local_description().await {
        return Ok(local_desc);
    }

    return Err("generate local_description failed!".to_owned());
}
