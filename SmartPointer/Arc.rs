use core::num;
use std::sync::Arc;
use std::thread::{self, AccessError};

fn RunArcBasic() {
    let number = Arc::new(10);

    let n1 = Arc::clone(&number);
    let n2 = Arc::clone(&number);
    let t1 = thread::spawn(move || {
        println!(" Thread 1 value is {}", n1);
    });

    let t2 = thread::spawn(move || {
        println!("THread 2 value {}", n2);
    });
    t1.join().unwrap();
    t2.join().unwrap();
    println!("Strong count = {}", Arc::strong_count(&number));
}

fn main() {
    RunArcBasic();
}
