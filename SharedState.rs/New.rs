use std::sync::Mutex;
use std::thread;

fn main() {
    let data = Mutex::new(Vec::new());
    let mut handles = vec![];

    for i in 1..=5 {
        let data_ref = &data;
        let handle = thread::spawn(move || {
            let mut v = data_ref.lock().unwrap();
            v.push(i);
        });
        handles.push(handle);
    }

    for h in handles { h.join(). unwrap()};
    println!("Result is {:?}" , *data.lock().unwrap());
}