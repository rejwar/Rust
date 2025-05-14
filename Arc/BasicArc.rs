use std::sync::Arc;
use std::thread;

fn main() {
    let SharedData = Arc::new("Hello from Arc!");

    let Thread1 = thread::spawn({
        let DataRef = Arc::clone(&SharedData);
        move || println!("{}", DataRef)
    });

    let Thread2 = thread::spawn({
        let DataRef = Arc::clone(&SharedData);
        move || println!("{}", DataRef)
    });

    Thread1.join().unwrap();
    Thread2.join().unwrap();
}
