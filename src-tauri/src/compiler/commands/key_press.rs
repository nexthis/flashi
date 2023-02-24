use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use enigo::*;

#[derive(Clone)]
pub struct KeyPress {}

impl Command for KeyPress {
    fn name(&self) -> String {
        "press".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let target = match arguments.get(0) {
            Some(val) => val.chars().next().unwrap(),
            None => return CommandResult::Error("Value is requared".to_string()),
        };

        let mut enigo = Enigo::new();
        enigo.key_click(Key::Layout(target));
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<KeyPress> {
    Box::new(KeyPress {})
}
