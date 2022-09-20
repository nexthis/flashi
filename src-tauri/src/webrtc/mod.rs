use crate::state::GlobalState;

#[tauri::command]
pub fn connect(state: tauri::State<GlobalState>) {
    println!("test: {}", state.test)
}
