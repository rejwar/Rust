use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx , rx) = mpsc::channel();
     thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        tx.send("Delayed Message ").unwrap();

     });

     match rx.recv_timeout(Duration::from_secs(2)) {
         Ok(msg) => println!("Received is {}" , msg),
         Err(e) => println!("TimeOut {}", e),

     }
}