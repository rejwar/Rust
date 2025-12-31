use std::thread::{self, JoinHandle};

fn main() {
    let Handle = thread::Builder::new()
        .name("Data- Process ".into())
        .spawn(|| {
            let current = thread::current();
            println!("THread Id : {:?}", current.name());
        })
        .unwrap();
    Handle.join().unwrap();
}
