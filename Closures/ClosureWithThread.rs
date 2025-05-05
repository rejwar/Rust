use std::thread;

fn main() {
    let Handle = thread::spawn(|| {
        println!("Running in a separate thread!");
    });

    Handle.join().unwrap();
}
