use super::tokenizer::tokens::Token;

pub fn run(tokens: Vec<Token>) {
    let mut line: Vec<Token> = Vec::new();

    for item in tokens.into_iter() {
        if let Token::EOL = item {
            println!("{:?}", line);
            line.clear();
            continue;
        }
        line.push(item);
    }

    // println!("{:?}", line());
    // println!("{:?}", line());
    // println!("{:?}", line());

    // println!("{:?}", lines.line());

    // println!("{:?}", lines.line());
    // for item in tokens {
    //     if let Token::EOL = item {
    //         break;
    //     }
    //     line.push(item)
    // }
    // println!("{:?}", line);
}
