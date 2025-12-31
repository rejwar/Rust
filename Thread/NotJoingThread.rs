use std::thread;

fn main() {
    thread::spawn(|| {
        println!("I am doing a important work");
    });
    println!("Main thread is end ");
}
