use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time;

use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use duckscript::types::runtime::StateValue;
use webrtc::data_channel::RTCDataChannel;

#[derive(Clone)]
pub struct Prompt {}

impl Command for Prompt {
    fn name(&self) -> String {
        "mobile_prompt".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        _arguments: Vec<String>,
        _state: &mut std::collections::HashMap<String, duckscript::types::runtime::StateValue>,
        _variables: &mut std::collections::HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<duckscript::types::instruction::Instruction>,
        _commands: &mut duckscript::types::command::Commands,
        _line: usize,
    ) -> CommandResult {
        //&dyn Any<Rc<RefCell<Arc<RTCDataChannel>>>>

        let data_channel: &StateValue = _state.get("data_channel").unwrap();

        let data_channel = match &data_channel {
            &StateValue::Any(value) => value.borrow(),
            _ => return CommandResult::Continue(Some("false".to_string())),
        };

        let data_channel = match data_channel.downcast_ref::<Arc<RTCDataChannel>>() {
            Some(value) => value,
            None => return CommandResult::Continue(Some("false".to_string())),
        };

        futures::executor::block_on(data_channel.send_text("XDDD".to_string())).unwrap();

        let last_message: &StateValue = _state.get("last_message").unwrap();

        let last_message = match &last_message {
            &StateValue::Any(value) => value.borrow(),
            _ => return CommandResult::Continue(Some("false".to_string())),
        };

        let last_message = match last_message.downcast_ref::<Arc<RwLock<String>>>() {
            Some(value) => value,
            None => return CommandResult::Continue(Some("false".to_string())),
        };

        let prev_message = last_message.read().unwrap().to_string();
        println!("message 1 ->>>>>>: {}", prev_message);

        while prev_message == last_message.read().unwrap().to_string() {
            println!(
                "loop 1 ->>>>>>: {}, {}",
                prev_message,
                last_message.read().unwrap()
            );
            thread::sleep(time::Duration::from_millis(200));
        }

        println!("message 2 ->>>>>>: {}", last_message.read().unwrap());

        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<Prompt> {
    Box::new(Prompt {})
}
