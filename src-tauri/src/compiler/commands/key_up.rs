use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use enigo::*;

#[derive(Clone)]
pub struct KeyUp {}

impl Command for KeyUp {
    fn name(&self) -> String {
        "key_up".to_string()
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
        enigo.key_up(Key::Layout(target));
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<KeyUp> {
    Box::new(KeyUp {})
}
