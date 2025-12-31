use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        println!(" working for new thread");
        thread::sleep(Duration::from_secs(2));
        println!("New thread work is done");
        "Successful"
    });

    println!("Main thread I am waiting here");
    let result = handle.join().unwrap();
    println!("Main thread &  The reuslt is {}", result);
}
