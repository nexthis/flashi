#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::compiler;
use app::registry;
use app::tray;
use app::webrtc;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            app.manage(tauri::async_runtime::block_on(async {
                webrtc::WebRtc::init(window).await
            }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            webrtc::connect,
            compiler::compile,
            registry::registry
        ])
        .system_tray(tray::make())
        .on_system_tray_event(tray::on_system_tray_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    std::process::exit(0);
}
