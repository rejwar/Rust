use std::thread;

fn main() {
    for i in 0..20 {
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(10));
            println!("Thread {} done ", i);
        });
    }
}
