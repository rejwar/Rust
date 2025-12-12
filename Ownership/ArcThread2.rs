use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(String::from("Thread safe"));
    let data_clone = Arc::clone(&data);

    thread::spawn(move || {
        println!("In thread {}", data_clone);
    })
    .join()
    .unwrap();
}
