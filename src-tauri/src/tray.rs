use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn make() -> SystemTray {
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");

    let tray_menu = SystemTrayMenu::new().add_item(exit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
        }

        SystemTrayEvent::MenuItemClick { tray_id, .. } => match tray_id.as_str() {
            "exit" => {
                app.exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
