use duckscript::types::{command::Commands, error::ScriptError};
use duckscriptsdk;

mod key_down;
mod key_press;
mod key_up;
mod log;
mod mouse_click;
mod mouse_down;
mod mouse_move;
mod mouse_move_relative;
mod mouse_up;
mod prompt;
mod typing;

pub fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(key_press::create())?;
    commands.set(key_up::create())?;
    commands.set(key_down::create())?;
    commands.set(typing::create())?;
    commands.set(mouse_move::create())?;
    commands.set(mouse_move_relative::create())?;
    commands.set(mouse_click::create())?;
    commands.set(mouse_up::create())?;
    commands.set(mouse_down::create())?;
    commands.set(log::create())?;
    commands.set(prompt::create())?;

    match duckscriptsdk::load(commands) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    Ok(())
}
