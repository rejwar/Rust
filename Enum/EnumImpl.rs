enum Message {
    Quit, 
    Move {x:i32 , y:i32},
    Write(String),
}

fn ProcessMessage(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } =>println!("Move to ({} , {})", x,y),
        Message::Write(text) => println!("Text: {}" , text),
    }
}

fn main () {
    ProcessMessage(Message::Move { x: 10, y: 20 });
    ProcessMessage(Message::Write(String::from("Hello !")));
}
