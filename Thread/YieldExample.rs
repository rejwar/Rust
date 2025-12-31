use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 0..100 {
            println!("THread A : {}", i);
            thread::yield_now();
        }
    });

    for i in 0..100 {
        println!("Main THread {}", i);
        thread::yield_now();
    }
}
