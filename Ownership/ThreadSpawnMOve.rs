use std::thread;

fn main() {
    let msg = String::from("Hello from Thread ");

    let handle = thread::spawn(move || {
        println!("Thread says: {}", msg);
    });

    handle.join().unwrap();
}
