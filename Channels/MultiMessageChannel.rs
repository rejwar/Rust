use std::sync::mpsc;
use std::thread;

fn main() {
    let (Sender, Receiver) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            Sender.send(format!("Message {}", i)).unwrap();
        }
    });

    while let Ok(Message) = Receiver.recv() {
        println!("Received: {}", Message);
    }
}
