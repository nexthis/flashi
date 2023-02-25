use duckscript::types::command::CommandResult;
use enigo::Key;

pub fn text_to_key(value: String) -> Result<Key, CommandResult> {
    if value.chars().count() <= 1 {
        return Ok(Key::Layout(value.chars().next().unwrap()));
    }

    match value.to_lowercase().as_str() {
        "alt" => return Ok(Key::Alt),
        "alt" => return Ok(Key::Alt),
        "backspace" => return Ok(Key::Backspace),
        "capslock" => return Ok(Key::CapsLock),
        "command" => return Ok(Key::Command),
        "control" => return Ok(Key::Control),
        "delete" => return Ok(Key::Delete),
        "downarrow" => return Ok(Key::DownArrow),
        "end" => return Ok(Key::End),
        "escape" => return Ok(Key::Escape),
        "f1" => return Ok(Key::F1),
        "f2" => return Ok(Key::F2),
        "f3" => return Ok(Key::F3),
        "f4" => return Ok(Key::F4),
        "f5" => return Ok(Key::F5),
        "f6" => return Ok(Key::F6),
        "f7" => return Ok(Key::F7),
        "f8" => return Ok(Key::F8),
        "f9" => return Ok(Key::F9),
        "f10" => return Ok(Key::F10),
        "f11" => return Ok(Key::F11),
        "f12" => return Ok(Key::F12),
        "f13" => return Ok(Key::F13),
        "f14" => return Ok(Key::F14),
        "f15" => return Ok(Key::F15),
        "f16" => return Ok(Key::F16),
        "f17" => return Ok(Key::F17),
        "f18" => return Ok(Key::F18),
        "f19" => return Ok(Key::F19),
        "f20" => return Ok(Key::F20),
        "home" => return Ok(Key::Home),
        "leftarrow" => return Ok(Key::LeftArrow),
        "meta" => return Ok(Key::Meta),
        "option" => return Ok(Key::Option),
        "pagedown" => return Ok(Key::PageDown),
        "pageup" => return Ok(Key::PageUp),
        "return" => return Ok(Key::Return),
        "rightarrow" => return Ok(Key::RightArrow),
        "shift" => return Ok(Key::Shift),
        "space" => return Ok(Key::Space),
        "super" => return Ok(Key::Super),
        "tab" => return Ok(Key::Tab),
        "uparrow" => return Ok(Key::UpArrow),
        "windows" => return Ok(Key::Windows),
        val => {
            let key = match val.parse::<u16>() {
                Ok(val) => val,
                Err(_) => return Err(CommandResult::Error("Value is not match".to_string())),
            };
            return Ok(Key::Raw(key));
        }
    }
}
