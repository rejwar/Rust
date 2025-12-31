use std::thread;
use std::time::Duration;

fn main() {
    println!("The program is going to start");

    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("Sub thread is working {}", i);

            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..=3 {
        println!(" Main thread is working {}", i);
        thread::sleep(Duration::from_secs(1));
    }

    handle.join().unwrap();
    println!("The work is done ");
}
