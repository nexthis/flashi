use std::fmt::Display;

mod tokenizer;

pub fn compile<T: Display>(value: T) {
    let value = value.to_string();
    let value = tokenizer::run(&value);
    println!("alive {}", value.to_string())
}

#[cfg(test)]
mod tests {
    use super::compile;
    #[test]
    fn it_works() {
        let code = r#"
            PRESS a
            MOVE 5 5 
            PRESS WIN 
        "#;
        compile(code);
    }
}
