use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time};

use super::utils::inputs::send;

#[derive(Clone)]
pub struct Press {}

impl Command for Press {
    fn name(&self) -> String {
        "move".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let x = match arguments.get(0) {
            Some(val) => val,
            None => return CommandResult::Error("Value X is required".to_string()),
        };

        let y = match arguments.get(1) {
            Some(val) => val,
            None => return CommandResult::Error("Value Y is required".to_string()),
        };

        let x = match x.parse::<f64>() {
            Ok(val) => val,
            Err(_) => return CommandResult::Error("Value X has wrong format".to_string()),
        };

        let y = match y.parse::<f64>() {
            Ok(val) => val,
            Err(_) => return CommandResult::Error("Value Y has wrong format".to_string()),
        };

        println!("value: {} - {}", x, y);
        send(&EventType::MouseMove { x, y });
        // send(&EventType::KeyRelease(key));
        // //Key::Layout(())
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<Press> {
    Box::new(Press {})
}
