use std::sync::Arc;
use std::thread;

fn main() {
    let data = create_arc();
    thread::spawn(move || {
        println!("Thread safe {}", data);
    })
    .join()
    .unwrap();
}

fn create_arc() -> Arc<String> {
    Arc::new(String::from("Safe Data"));
}
