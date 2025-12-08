use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!(" I am new thread ");
    });

    handle.join().unwrap();
    println!(" i am new thread  now ");
}
