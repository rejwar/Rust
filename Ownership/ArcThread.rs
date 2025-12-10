use std::sync::Arc;
use std::thread;

fn main() {
    let shared_data = Arc::new(String::from("THread safe Data"));

    let mut handles = vec![];

    for _ in 0..3 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            println!(" Thread sees {}", data_clone);
        });

        handles.push(handles);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
