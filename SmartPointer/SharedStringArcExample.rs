use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let ShareData: Arc<String> = Arc::new(String::from("Concurrency"));

    let ThreadOne = {
        let CloneData = Arc::clone(&ShareData);

        thread::spawn(move || {
            println!("Thread One: {}", CloneData);
        })
    };

    let ThreadTwo = {
        let CloneData = Arc::clone(&ShareData);

        thread::spawn(move || {
            println!("Thread Two: {}", CloneData);
        })
    };

    ThreadOne.join().unwrap();
    ThreadTwo.join().unwrap();
}
