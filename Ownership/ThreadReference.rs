use std::sync::Arc;
use std::thread;

fn main() {
    let SharedData: Arc<String> = Arc::new(String::from("Thread Safe Reference"));

    let ThreadOne = thread::spawn({
        let CloneData = Arc::clone(&SharedData);
        move || println!("{}", CloneData)
    });

    ThreadOne.join().unwrap();
}
