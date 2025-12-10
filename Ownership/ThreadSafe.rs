use std::sync::Arc;
use std::thread;

fn main() {
    let data = get_thread_safe_data();

    thread::spawn(move || {
        println!("In thread {}", data);
    })
    .join()
    .unwrap();
}

fn get_thread_safe_data() -> Arc<String> {
    Arc::new(String::from("THread safe Data"))
}
