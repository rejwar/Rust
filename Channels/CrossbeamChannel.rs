use crossbeam::channel::unbounded;
use std::thread;

fn main() {
    let (Sender, Receiver) = unbounded();

    thread::spawn(move || {
        Sender.send("Crossbeam Channel Message").unwrap();
    });

    println!("Received: {}", Receiver.recv().unwrap());
}
