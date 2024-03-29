use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use enigo::*;

#[derive(Clone)]
pub struct Typing {}

impl Command for Typing {
    fn name(&self) -> String {
        "typing".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let target = match arguments.get(0) {
            Some(val) => val,
            None => return CommandResult::Error("Value is requared".to_string()),
        };

        let mut enigo = Enigo::new();
        enigo.key_sequence(target);
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<Typing> {
    Box::new(Typing {})
}
