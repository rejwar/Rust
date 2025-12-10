enum Message {
    Write(String),
}

fn main() {
    let msg = Message::Write(String::from("Hello"));

    if let Message::Write(text) = msg {
        println!("Content {}", text);
    }
}
