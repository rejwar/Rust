use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx ,rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from thread ") .unwrap();
    });
    let msg = rx.recv().unwrap();
    println!("Received in {}" , msg);
}