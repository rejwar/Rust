let msg = Message::Write(String::from("Hello"));

if let Message::Write(content) = msg {
    println!("Message: {}", content);
}
