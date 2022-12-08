use duckscript;
use duckscript::runner;
use duckscript::types;
use duckscript::types::error::ScriptError;

use std::fmt::Display;

mod commands;

#[tauri::command]
pub fn compile(value: String) -> Result<String, String> {
    let mut context = types::runtime::Context::new();

    commands::load(&mut context.commands);

    match runner::run_script(value.to_string().as_str(), context) {
        Ok(_) => return Ok("succes".to_string()),
        Err(err) => return Err(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::compile;

    #[test]
    fn it_works_simple() {
        let code = r#"
            out = set "Hello World"

            # This will print: "The out variable holds the value: Hello World"
            echo The out variable holds the value: ${out}
            press A 
            press A 
            press A 
            press A 
            press A 
            press A 
            press A 
            move 50.54 50 
        "#;

        let now = Instant::now();
        compile(code.to_string());
        let elapsed_time = now.elapsed();

        println!("Running it took {:?} seconds.", elapsed_time);
    }
}
