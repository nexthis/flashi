use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time};

use super::utils::inputs::key_to_keycode;
use super::utils::inputs::send;

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
