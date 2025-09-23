use std::{result, thread};

fn main() {
    let handle = thread::spawn(|| {
        let sum: i32 = (1..=100).sum();
        sum
        
    });

    let result = handle.join().unwrap();
    println!("sum from thread {}", result);
}