use enigo::Key;

pub fn text_to_key(value: String) -> Key {
    if value.chars().count() <= 1 {
        return Key::Layout(value.chars().next().unwrap());
    }

    match value.to_lowercase().as_str() {
        "alt" => return Key::Alt,
        "alt" => return Key::Alt,
        "backspace" => return Key::Backspace,
        "capslock" => return Key::CapsLock,
        "command" => return Key::Command,
        "control" => return Key::Control,
        "delete" => return Key::Delete,
        "downarrow" => return Key::DownArrow,
        "end" => return Key::End,
        "escape" => return Key::Escape,
        "f1" => return Key::F1,
        "f2" => return Key::F2,
        "f3" => return Key::F3,
        "f4" => return Key::F4,
        "f5" => return Key::F5,
        "f6" => return Key::F6,
        "f7" => return Key::F7,
        "f8" => return Key::F8,
        "f9" => return Key::F9,
        "f10" => return Key::F10,
        "f11" => return Key::F11,
        "f12" => return Key::F12,
        "f13" => return Key::F13,
        "f14" => return Key::F14,
        "f15" => return Key::F15,
        "f16" => return Key::F16,
        "f17" => return Key::F17,
        "f18" => return Key::F18,
        "f19" => return Key::F19,
        "f20" => return Key::F20,
        "home" => return Key::Home,
        "leftarrow" => return Key::LeftArrow,
        "meta" => return Key::Meta,
        "option" => return Key::Option,
        "pagedown" => return Key::PageDown,
        "pageup" => return Key::PageUp,
        "return" => return Key::Return,
        "rightarrow" => return Key::RightArrow,
        "shift" => return Key::Shift,
        "space" => return Key::Space,
        "super" => return Key::Super,
        "tab" => return Key::Tab,
        "uparrow" => return Key::UpArrow,
        "windows" => return Key::Windows,
        val => {
            let key = match val.parse::<u16>() {
                Ok(val) => val,
                Err(_) => return Key::Raw(0x12),
            };
            return Key::Raw(key);
        }
    }
}
