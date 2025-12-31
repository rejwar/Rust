use std::thread;
use std::time::Duration;

fn main() {
    let message = String::from("Rust thread message");
    let handle = thread::spawn(move || {
        for i in 1..=5 {
            println!(" {} number is {}", message, i);
            thread::sleep(Duration::from_millis(400));
        }
    });

    for i in 1..=3 {
        println!("main thread counter {}", i);
        thread::sleep(Duration::from_millis(600));
    }

    handle
        .join()
        .expect("Thread can not be completely successfully");
    println!("THe program has completed successfully");
}
