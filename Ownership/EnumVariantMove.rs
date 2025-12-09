enum Message {
    Quit,
    Write(String),
}

fn main() {
    let msg = Message::Write(String::from("Hello Rust"));

    match msg {
        Message::Quit => println!("Quit"),
        Message::Write(text) => {
            println!("GOt message {}", text);
        }
    }
}
