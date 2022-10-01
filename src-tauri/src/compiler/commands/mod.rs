use duckscript::types::{command::Commands, error::ScriptError};
use duckscriptsdk;

mod press;

pub fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(press::create())?;

    match duckscriptsdk::load(commands) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    Ok(())
}
