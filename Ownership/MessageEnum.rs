enum Message {
    Quit,
    Move { x: i32, y: i32 },
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to {x}, {y}"),
    }
}
