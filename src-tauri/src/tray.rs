use tauri::{AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn make() -> SystemTray {
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");

    let tray_menu = SystemTrayMenu::new().add_item(exit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { tray_id, id, .. } => match id.as_str() {
            "exit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
