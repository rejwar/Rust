use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("spawned: {}", i);
        }
    });

    for i in 1..=3 {
        println!("main: {}", i);
    }

    handle.join().unwrap();
}
