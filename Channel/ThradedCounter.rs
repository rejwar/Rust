use std::sync::mpsc;
use std::thread;

fn main() {
    let(tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..= 5{
            tx.send(i) .unwrap();
        }
    });

    for val in rx.iter().take(5) {
        println!("Counter {}" , val);
    }
}