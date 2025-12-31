use std::thread;
use std::time::Duration;

fn main() {
    let builder = thread::Builder::new()
        .name("Worker thread Aplpha ".into())
        .stack_size(32 * 1024);

    let handle = builder
        .spawn(|| {
            let current_thread = thread::current();
            println!("Hello from {}", current_thread.name().unwrap_or("Unnamed"));

            if true {
                panic!("Something went wrong in Alpha");
            }
        })
        .unwrap();

    let result = handle.join();

    if let Err(_) = result {
        println!("the main thread caught a panic from a named thread");
    }
}
