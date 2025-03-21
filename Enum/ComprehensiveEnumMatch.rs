enum Signal {
    Red,
    Yellow,
    Green,
}

fn main() {
    let signal = Signal::Green;
    match signal {
        Signal::Red => println!("Stop!"),
        Signal::Yellow => println!("Get ready to stop."),
        Signal::Green => println!("Go!"),
    }
}
