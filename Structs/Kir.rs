#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}


impl Message {
    fn process(&self) {
        match self {
            Message::Quit => {
                println!("System shutting down...");
            }
            Message::Move { x, y } => {
                println!("Moving to position: ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("Writing message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to RGB({}, {}, {})", r, g, b);
            }
        }
    }
}

fn main() {
    let messages = vec![
        Message::Write(String::from("Hello, Md.!")),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    for msg in messages {
        msg.process();
    }
}
