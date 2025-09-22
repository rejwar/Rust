use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Child thread {}", i);
        }
    });

    for i in 1..=5 {
        println!("Main thread {}", i);
    }

    handle.join().unwrap();
}