use std::thread;
fn main() {
    let msg = String::from("Hi thread");
    thread::spawn(move || println!("{}", msg)).join().unwrap();
}
