use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        10 + 20
    });

    let result = handle.join().unwrap();
    println!("Result from thread is {}", result);
}