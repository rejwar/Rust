use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..=5 {
            println!("Calling from new thread . I am working here {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Main threrad , I am waiting here for few times");
    thread::sleep(Duration::from_secs(1));

    println!("Main thread end .  The full process will end soon");
}
