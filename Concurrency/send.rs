use std::thread;

fn main() {
    let s = String::from("Hello");

    let handle = thread::spawn(move || {
        println!("{}" , s);
    });

    handle.join().unwrap();
}