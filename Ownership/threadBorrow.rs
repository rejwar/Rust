use std::thread;

fn main() {
    let msg = String::from("Scoped Borrow");

    thread::scope(|s| {
        s.spawn(|| {
            println!("Thread sees {}", &msg);
        });
    });
}
