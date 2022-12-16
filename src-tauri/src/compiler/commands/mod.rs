use duckscript::types::{command::Commands, error::ScriptError};
use duckscriptsdk;

mod mouse_move;
mod mouse_move_relative;
mod press;
mod utils;

pub fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(press::create())?;
    commands.set(mouse_move::create())?;
    commands.set(mouse_move_relative::create())?;

    match duckscriptsdk::load(commands) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    Ok(())
}
