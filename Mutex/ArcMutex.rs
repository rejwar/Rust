use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let Counter = Arc::new(Mutex::new(0));
    let mut Handles = vec![];

    for _ in 0..5 {
        let CounterRef = Arc::clone(&Counter);
        let Handle = thread::spawn(move || {
            let mut Lock = CounterRef.lock().unwrap();
            *Lock += 1;
        });
        Handles.push(Handle);
    }

    for Handle in Handles {
        Handle.join().unwrap();
    }

    println!("Final Counter Value: {}", *Counter.lock().unwrap());
}
