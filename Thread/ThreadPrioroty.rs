use std::thread;

fn main() {
    thread::spawn(||{
        unsafe  { libc::nice(19);}
        println!("THis is thread runs with lower prority");
    )}.join().unwrap();
}