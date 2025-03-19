use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("Hello from the thread!");
        tx.send(message).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
