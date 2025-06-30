enum Message {
    Quit,
    Move{x: i32 , y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message){
    match msg {
        Message::Quit =>println!("Quit"),
        Message::Move { x, y } => println!("Moved to {} {}",x,y),
        Message::Write(text) => println!("Text Message {}", text),
        Message::ChangeColor(r,g,b) => println!("Change color r , g , b {}, {} {}",r,g,b),
    }
}

fn main() {

    let msg2 = Message::Move { x: 10, y: 10 };
    let msg3 = Message::Write(String::from("Hello World"));
    let msg4 = Message::ChangeColor(255, 6, 10);


    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
}
