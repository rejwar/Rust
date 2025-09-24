use std::thread; 


fn main() {
    let handle = thread::spawn(|| {
        let sum: i32 = (1..=100).sum();
        sum
    });

    let result = handle.join().unwrap();
    println!("Sum from thread {}", result);
}