

enum Token<'a> {
    Word(&'a str),
    Number(i32),
}

fn main() {
    let text = "hello";
    let token = Token::Word(text);

    match token {
        Token::Word(s) => println!("Word: {}", s),
        Token::Number(n) => println!("Number: {}", n),
    }
}
