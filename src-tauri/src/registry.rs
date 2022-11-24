use machineid_rs::HWIDComponent;
use machineid_rs::{Encryption, IdBuilder};
use sysinfo::{System, SystemExt};

#[derive(serde::Serialize)]
pub struct Output {
    key: String,
    name: String,
    os: String,
}

#[tauri::command]
pub fn registry() -> Output {
    let mut builder = IdBuilder::new(Encryption::SHA256);
    let mut sys = System::new();

    builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUCores);

    let key = builder.build("registry").unwrap();
    let name = sys.host_name().unwrap();
    let os = sys.name().unwrap();

    Output { key, name, os }
}
