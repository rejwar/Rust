enum Signal {
    Start,
    Stop,
    Pause,
}

fn main() {
    let signal = Signal::Start;
    match signal {
        Signal::Start => println!("Signal: Start"),
        Signal::Stop => println!("Signal: Stop"),
        Signal::Pause => println!("Signal: Pause"),
    }
}
