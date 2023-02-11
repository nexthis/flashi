use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use duckscript::types::command::Command;
use duckscript::types::command::CommandResult;
use duckscript::types::runtime::StateValue;
use internal::key;
use webrtc::data_channel::RTCDataChannel;

#[derive(Clone)]
pub struct Log {}

impl Command for Log {
    fn name(&self) -> String {
        "mobile_log".to_string()
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

        println!("test: data_channel 1");

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

        CommandResult::Continue(Some("true".to_string()))
    }
}

pub fn create() -> Box<Log> {
    Box::new(Log {})
}
