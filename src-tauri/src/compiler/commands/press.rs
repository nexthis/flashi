use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use internal::key;

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
            Some(val) => val.chars().next().unwrap(),
            None => return CommandResult::Error("Value is requared".to_string()),
        };

        key::tap(&key::Character(target), &[], 0, 0);
        //Key::Layout(())
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<Press> {
    Box::new(Press {})
}
