use std::thread;

fn main() {
    let msg = String::from("THread safety");

    thread::scope(|s| {
        s.spawn(|| {
            println!("Borrowed inside scope {}", &msg);
        });
    });
}
