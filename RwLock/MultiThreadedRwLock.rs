use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let SharedData = Arc::new(RwLock::new(100));
    let mut Handles = vec![];

    for _ in 0..5 {
        let SharedRef = Arc::clone(&SharedData);
        let Handle = thread::spawn(move || {
            let ReadLock = SharedRef.read().unwrap();
            println!("Thread Read: {}", *ReadLock);
        });
        Handles.push(Handle);
    }

    {
        let mut WriteLock = SharedData.write().unwrap();
        *WriteLock += 50;
    }

    for Handle in Handles {
        Handle.join().unwrap();
    }

    println!("Final Value: {}", *SharedData.read().unwrap());
}
