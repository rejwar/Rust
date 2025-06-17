// Question: How can enums hold extra data like numbers or strings?

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn UseMessage() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("The app will quit."),
        Message::Move { x, y } => println!("Move to position: ({}, {})", x, y),
        Message::Write(text) => println!("Writing message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}

fn main() {
    UseMessage();
}
