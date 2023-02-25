use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use enigo::*;

use crate::compiler::keys::text_to_key;

#[derive(Clone)]
pub struct Press {}

impl Command for Press {
    fn name(&self) -> String {
        "key_down".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let target = match arguments.get(0) {
            Some(val) => val,
            None => return CommandResult::Error("Value is requared".to_string()),
        };

        let target = match text_to_key(target.clone()) {
            Ok(it) => it,
            Err(err) => return err,
        };

        let mut enigo = Enigo::new();
        enigo.key_down(target);
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<Press> {
    Box::new(Press {})
}
