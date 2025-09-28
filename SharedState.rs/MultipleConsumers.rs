use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx , rx) = mpsc::channel();

    for i in 0..7 {
        let txc = tx.clone();
        thread::spawn(move || {
            txc.send(format!("Message is {}" ,i)).unwrap();

        });
    }
        drop(tx);

        for msg in rx {
            println!("Got {}" , msg);
        
    }
}