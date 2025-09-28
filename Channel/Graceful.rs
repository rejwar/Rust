use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx , rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("start").unwrap();
        tx.send("shutdown").unwrap();

    });

    for msg in rx.iter() {
        println!("Received {}" , msg);
        if msg == "shutdown" {
            println!("shutting down");
            break;
        }
    }
}