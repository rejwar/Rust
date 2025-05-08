use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (Sender, Receiver) = mpsc::channel();

    thread::spawn(move || {
        Sender.send("Hello from thread!").unwrap();
    });

    let Message = Receiver.recv().unwrap();
    println!("Received: {}", Message);
}
