use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time};

#[derive(Clone)]
pub struct Press {}

impl Command for Press {
    fn name(&self) -> String {
        "press".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let target = match arguments.get(0) {
            Some(val) => val,
            None => return CommandResult::Error("Value is requared".to_string()),
        };

        let key = key_to_keycode(target);

        println!("value: {}", target);
        send(&EventType::KeyPress(key));
        send(&EventType::KeyRelease(key));
        //Key::Layout(())
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<Press> {
    Box::new(Press {})
}

fn key_to_keycode(key: &String) -> Key {
    return match key.to_lowercase().as_str() {
        "alt" => Key::Alt,
        "altgr" => Key::AltGr,
        "backspace" => Key::Backspace,
        "capslock" => Key::CapsLock,
        "controlleft" => Key::ControlLeft,
        "controlright" => Key::ControlRight,
        "delete" => Key::Delete,
        "downarrow" => Key::DownArrow,
        "end" => Key::End,
        "escape" => Key::Escape,
        "f1" => Key::F1,
        "f10" => Key::F10,
        "f11" => Key::F11,
        "f12" => Key::F12,
        "f2" => Key::F2,
        "f3" => Key::F3,
        "f4" => Key::F4,
        "f5" => Key::F5,
        "f6" => Key::F6,
        "f7" => Key::F7,
        "f8" => Key::F8,
        "f9" => Key::F9,
        "home" => Key::Home,
        "leftarrow" => Key::LeftArrow,
        "metaleft" => Key::MetaLeft,
        "metaright" => Key::MetaRight,
        "pagedown" => Key::PageDown,
        "pageup" => Key::PageUp,
        "return" => Key::Return,
        "rightarrow" => Key::RightArrow,
        "shiftleft" => Key::ShiftLeft,
        "shiftright" => Key::ShiftRight,
        "space" => Key::Space,
        "tab" => Key::Tab,
        "uparrow" => Key::UpArrow,
        "printscreen" => Key::PrintScreen,
        "scrolllock" => Key::ScrollLock,
        "pause" => Key::Pause,
        "numlock" => Key::NumLock,
        "backquote" => Key::BackQuote,
        "num1" => Key::Num1,
        "num2" => Key::Num2,
        "num3" => Key::Num3,
        "num4" => Key::Num4,
        "num5" => Key::Num5,
        "num6" => Key::Num6,
        "num7" => Key::Num7,
        "num8" => Key::Num8,
        "num9" => Key::Num9,
        "num0" => Key::Num0,
        "minus" => Key::Minus,
        "equal" => Key::Equal,
        "q" => Key::KeyQ,
        "w" => Key::KeyW,
        "e" => Key::KeyE,
        "r" => Key::KeyR,
        "t" => Key::KeyT,
        "y" => Key::KeyY,
        "u" => Key::KeyU,
        "i" => Key::KeyI,
        "o" => Key::KeyO,
        "p" => Key::KeyP,
        "leftbracket" => Key::LeftBracket,
        "rightbracket" => Key::RightBracket,
        "a" => Key::KeyA,
        "s" => Key::KeyS,
        "d" => Key::KeyD,
        "f" => Key::KeyF,
        "g" => Key::KeyG,
        "h" => Key::KeyH,
        "j" => Key::KeyJ,
        "k" => Key::KeyK,
        "l" => Key::KeyL,
        "semicolon" => Key::SemiColon,
        "quote" => Key::Quote,
        "backslash" => Key::BackSlash,
        "intlbackslash" => Key::IntlBackslash,
        "z" => Key::KeyZ,
        "x" => Key::KeyX,
        "c" => Key::KeyC,
        "v" => Key::KeyV,
        "b" => Key::KeyB,
        "n" => Key::KeyN,
        "m" => Key::KeyM,
        "comma" => Key::Comma,
        "dot" => Key::Dot,
        "slash" => Key::Slash,
        "insert" => Key::Insert,
        "kpreturn" => Key::KpReturn,
        "kpminus" => Key::KpMinus,
        "kpplus" => Key::KpPlus,
        "kpmultiply" => Key::KpMultiply,
        "kpdivide" => Key::KpDivide,
        "kp0" => Key::Kp0,
        "kp1" => Key::Kp1,
        "kp2" => Key::Kp2,
        "kp3" => Key::Kp3,
        "kp4" => Key::Kp4,
        "kp5" => Key::Kp5,
        "kp6" => Key::Kp6,
        "kp7" => Key::Kp7,
        "kp8" => Key::Kp8,
        "kp9" => Key::Kp9,
        "kpdelete" => Key::KpDelete,
        "function" => Key::Function,
        _ => Key::Unknown(23),
    };
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}
