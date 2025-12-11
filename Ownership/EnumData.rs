enum Message {
    Write(String),
}

fn main() {
    let msg = Message::Write(String::from("Hello"));

    if let Message::Write(content) = msg {
        println!("Content {}", content);
    }
}
