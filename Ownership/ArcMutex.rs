use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let c = Arc::clone(&counter);

    thread::spawn(move || {
        let mut num = c.lock().unwrap();
        *num += 1;
    })
    .join()
    .unwrap();

    println!(" Result {:?}", counter);
}
