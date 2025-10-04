enum  Action {
    Quit,
    Move {x: i32 , y: i32},
    Write (String),
}

fn main() {
    let act = Action::Write(String::from("hello"));

    match act {
        Action::Move { x, y } =>  println!("Moving to forward {} {} ",x,y),
        _=> println!("Action ignored")
    }
}