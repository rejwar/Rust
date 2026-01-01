use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        println!("This will probably never print");
    });
    println!(" Main thread is exixting Immediately ");
}
