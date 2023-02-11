use duckscript::types::{command::Commands, error::ScriptError};
use duckscriptsdk;

mod log;
mod mouse_click;
mod mouse_move;
mod mouse_move_relative;
mod press;

pub fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(press::create())?;
    commands.set(mouse_move::create())?;
    commands.set(mouse_move_relative::create())?;
    commands.set(mouse_click::create())?;
    commands.set(log::create())?;

    match duckscriptsdk::load(commands) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    Ok(())
}
