use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let Counter = Arc::new(Mutex::new(0));
    let mut Handles = vec![];

    for _ in 0..5 {
        let Counter = Arc::clone(&Counter);
        let Handle = thread::spawn(move || {
            let mut Num = Counter.lock().unwrap();
            *Num += 1;
        });
        Handles.push(Handle);
    }

    for Handle in Handles {
        Handle.join().unwrap();
    }

    println!("Final Counter Value: {}", *Counter.lock().unwrap());
}
