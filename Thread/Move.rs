use std::thread;

fn main() {
    let msg = String::from("Hello");

    let handle = thread::spawn(move || {
        println!("Thread is telling  {}", msg);
    });

    handle.join().unwrap();
}
