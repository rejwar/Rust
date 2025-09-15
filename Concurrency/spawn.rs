use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("The child thread is {}" , i);
        }
    });

    println!("main thread is done ...");
    handle.join().unwrap();
}