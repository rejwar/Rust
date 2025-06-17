enum Message {
    Quit,
    Move{x:i32 , y:i32},
    Write(String),
    ChangeColor(i32 , i32 ,i32 ),
}

fn main() {
    let msg1: Message = Message::Quit;
    let msg2: Message = Message::Move { x: 10, y: 12 };
    let msg3: Message = Message::Write(String::from("Hello"));
    let msg4: Message = Message::ChangeColor(255, 0, 0);
}
