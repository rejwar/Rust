use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx , rx ) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        tx.send("Quick message").unwrap();

    });

    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("Received {}" , msg);
                break;
            }
            Err(_) => {
                println!("Waiting ... ?");
                thread::sleep(Duration::from_millis(500));
            }
        }
    }
}