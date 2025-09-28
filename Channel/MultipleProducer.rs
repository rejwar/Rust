
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx , rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send("from tx ").unwrap();

    });

    thread::spawn(move || {
        tx1.send("from tx1 ").unwrap();
    });

    for _ in 0..2 {
        println!("Received {}" , rx.recv().unwrap());
    }
}