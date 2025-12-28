use std::sync::{Arc, Mutex};
use std::thread::{self, AccessError};

fn RunArcMutexCounter() {
    let counter = Arc::new(Mutex::new(10));

    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final counter value = {}", *counter.lock().unwrap());
}

fn main() {
    RunArcMutexCounter();
}
