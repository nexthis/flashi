use duckscript::runner;
use duckscript::types;
use duckscript::types::error::ScriptError;
use duckscriptsdk;
use std::fmt::Display;

pub fn compile<T: Display>(value: T) -> Result<duckscript::types::runtime::Context, ScriptError> {
    let mut context = types::runtime::Context::new();


    match duckscriptsdk::load(&mut context.commands) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    runner::run_script(value.to_string().as_str(), context)
}

#[cfg(test)]
mod tests {
    use super::compile;

    #[test]
    fn it_works() {
        let code = r#"
            PRESS    a
            SLEEP 5000
            MOVE 5%, 5 

            FOR RANGE 5,4
                 PRESS    a
                SLEEP 5000
            ENDFOR 

            PRESS WIN 
        "#;
        compile(code);
    }

    #[test]
    fn it_works_simple() {
        let code = r#"
            out = set "Hello World"

            # This will print: "The out variable holds the value: Hello World"
            echo The out variable holds the value: ${out}
        "#;
        compile(code);
    }
}
