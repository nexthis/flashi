#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use app::compiler;
use app::registry;
use app::tray;
use app::webrtc;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

#[derive(Clone, serde::Serialize)]
struct SingleInstancePayload {
    args: Vec<String>,
    cwd: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", SingleInstancePayload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
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
