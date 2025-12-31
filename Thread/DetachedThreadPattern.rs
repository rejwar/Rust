use std::thread;

use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..=10 {
            println!("Background Task {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    println!(" Main thread is going ahead without waiting ");
    thread::sleep(Duration::from_secs(3));
    println!("Main thread is end detacehed will off autometically");
}
