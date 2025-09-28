use std::sync::{mpsc, Arc, Mutex};
use std::sync::{Arc::Mutex::mpsc};
use std::thread;

fn main() {
    let (tx , rx) = mpsc::channel();
    let message = Arc::new(Mutex::new(Vec::new()));
    let message_clone = Arc::clone(&message);

    for i in 0..3 {
        let tx_clone = tx.clone();
        tx_clone.send(format!("Message from thread {}" , i)).unwrap();


        drop(tx);

        for msg in rx.iter() {
            let mut vec = messages_clone.lock().unwrap();
            vec.push
            (msf);
        }

        println!("All message {:?} , " , message , lock() . unwrap()); 
    }
}