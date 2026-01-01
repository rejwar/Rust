use std::thread;

fn main() {
    let t1 = thread::spawn(|| {
        println!("Thread 1: Hello from the other side");
    });

    let t2 = thread::spawn(|| {
        println!("THread 2: greetings from Here ");
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
