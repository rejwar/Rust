use std::sync::{Arc , Mutex};
use std::{thread, vec};

fn main() {
    let shared_list = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];


    for i in 0..5 {
        let list_clone = Arc::clone(&shared_list);
        let handle = thread::spawn(move || {
            let mut vec = list_clone.lock().unwrap();
            vec.push(i);
            println!("Thread  {} pushed value is {}" , i , i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_list = shared_list.lock().unwrap();
}