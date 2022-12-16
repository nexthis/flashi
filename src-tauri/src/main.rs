#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::compiler;
use app::registry;
use app::tray;
use app::webrtc;

fn main() {
    tauri::Builder::default()
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
