use duckscript::runner;
use duckscript::types;
use duckscript::types::error::ScriptError;

use std::fmt::Display;

mod commands;

pub fn compile<T: Display>(value: T) -> Result<duckscript::types::runtime::Context, ScriptError> {
    let mut context = types::runtime::Context::new();

    commands::load(&mut context.commands);

    runner::run_script(value.to_string().as_str(), context)
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
        "#;

        let now = Instant::now();
        compile(code);
        let elapsed_time = now.elapsed();

        println!("Running it took {:?} seconds.", elapsed_time);
    }
}
