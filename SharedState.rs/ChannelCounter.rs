use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx , rx ) = mpsc::channel();

    for _ in 0..=10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(1) . unwrap();
        });
    }

    drop(tx);

    let sum: i32 = rx.iter().sum();
    println!("Total {}" , sum);
}