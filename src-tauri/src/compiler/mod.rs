use std::fmt::Display;

mod commands;
mod tokenizer;
mod tokens;

pub fn compile<T: Display>(value: T) {
    let value = value.to_string();
    let values = tokenizer::run(value);
    println!("{:?}", values)
}

#[cfg(test)]
mod tests {
    use super::compile;

    #[test]
    fn it_works() {
        let code = r#"
            PRESS    a
             
              
            MOVE 5    5 


            PRESS  WIN 
            

            
        "#;
        compile(code);
    }
}
