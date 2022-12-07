use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time};

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
            Some(val) => val,
            None => return CommandResult::Error("Value is requared".to_string()),
        };

        let key = target.chars().nth(0).unwrap();

        send(&EventType::KeyPress(Key::KeyS));
        send(&EventType::KeyRelease(Key::KeyS));
        //Key::Layout(())
        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<Press> {
    Box::new(Press {})
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}
