use std::fmt::Display;

mod commands;
mod parser;
mod tokenizer;

pub fn compile<T: Display>(value: T) {
    let value = value.to_string();
    let values = tokenizer::run(value);

    parser::run(values);
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
            PRESS a
            SLEEP 500
            MOVE 5, 55.5
        "#;
        compile(code);
    }
}
