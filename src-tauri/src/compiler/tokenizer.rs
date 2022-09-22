use std::fmt::Display;

use super::tokens::Token;

pub fn run<T: Display>(value: T) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let value = normalizer(value.to_string());

    for line in value.lines() {
        let mut chars = line.chars();
        loop {
            let mut char = match chars.next() {
                Some(val) => val,
                None => break,
            };

            if char.is_alphabetic() {
                let mut value = String::new();
                while char.is_alphabetic() {
                    value.push(char);
                    char = match chars.next() {
                        Some(val) => val,
                        None => break,
                    };
                }
                result.push(Token::String(value));
                continue;
            }

            if char.is_numeric() {
                let mut value = String::new();
                while char.is_numeric() {
                    value.push(char);
                    char = match chars.next() {
                        Some(val) => val,
                        None => break,
                    };
                }
                result.push(Token::Numeric {
                    raw: value,
                    variant: super::tokens::NumericTypes::Decimal,
                });
                continue;
            }
        }
        
        //Add EOL when is not duplicate 
        if let Some(val) = result.last() {
            if let Token::EOL = val {
            } else {
                result.push(Token::EOL);
            }
        }
    }

    result
}

fn normalizer(value: String) -> String {
    let mut new_str = value.trim().to_owned();
    let mut prev = ' '; // The initial value doesn't really matter
    new_str.retain(|ch| {
        let result = ch != ' ' || prev != ' ';
        prev = ch;
        result
    });
    new_str
}
