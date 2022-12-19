use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;

use internal::mouse::{click, Button};

#[derive(Clone)]
pub struct MouseClick {}

impl Command for MouseClick {
    fn name(&self) -> String {
        "move_click".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let value = match arguments.get(0) {
            Some(val) => val,
            None => {
                return CommandResult::Error(
                    "Value left or right or middle is required".to_string(),
                )
            }
        };

        let button = match value.to_lowercase().as_str() {
            "left" => Button::Left,
            "right" => Button::Right,
            "middle" => Button::Middle,
            _ => {
                return CommandResult::Error(
                    "Incorrect value, correct value: left, right or middle ".to_string(),
                )
            }
        };

        println!("value: {}", value);
        click(button, None);
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<MouseClick> {
    Box::new(MouseClick {})
}
