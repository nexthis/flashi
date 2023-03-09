use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;

use enigo::*;

#[derive(Clone)]
pub struct Scroll {}

impl Command for Scroll {
    fn name(&self) -> String {
        "scroll".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let x = match arguments.get(0) {
            Some(val) => val,
            None => return CommandResult::Error("Value X is required".to_string()),
        };

        let x = match x.parse::<i32>() {
            Ok(val) => val,
            Err(_) => return CommandResult::Error("Value X has wrong format".to_string()),
        };

        let y = arguments.get(1);

        let mut enigo = Enigo::new();
        println!("scroll: {} {:?}", x, y);

        enigo.mouse_scroll_x(x);

        if let Some(y) = y {
            let y = match y.parse::<i32>() {
                Ok(val) => val,
                Err(_) => return CommandResult::Error("Value Y has wrong format".to_string()),
            };
            enigo.mouse_scroll_y(y);
        }

        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<Scroll> {
    Box::new(Scroll {})
}
