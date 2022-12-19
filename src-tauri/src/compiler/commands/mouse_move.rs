use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;

use internal::geometry::Point;
use internal::mouse::move_to;

#[derive(Clone)]
pub struct MouseMove {}

impl Command for MouseMove {
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
        move_to(Point { x, y });
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<MouseMove> {
    Box::new(MouseMove {})
}
