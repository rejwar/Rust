enum Msg {
    Quit,
    Move(i32),
}

fn main() {
    let m = Msg::Move(5);

    match m {
        Msg::Move(x) => println!(" {}", x),
        _ => {}
    }
}
