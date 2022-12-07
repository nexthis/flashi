use std::thread;
use std::time::Duration;

use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use enigo::{Enigo, Key, KeyboardControllable};

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
        let mut enigo = Enigo::new();

        let target = match arguments.get(0) {
            Some(val) => val,
            None => return CommandResult::Error("Value is requared".to_string()),
        };

        let key = target.chars().nth(0).unwrap();
        println!("Key::Layout: {}", key);

        enigo.key_down(Key::Layout(key));
        thread::sleep(Duration::from_secs(1));
        enigo.key_up(Key::Layout(key));
        //Key::Layout(())
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<Press> {
    Box::new(Press {})
}
