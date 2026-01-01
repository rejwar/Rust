use std::thread;

fn main() {
    thread::spawn(|| {
        println!(" Random number generation logic goes here ");
    })
    .join()
    .unwrap();
}
