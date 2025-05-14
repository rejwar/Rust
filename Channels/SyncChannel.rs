use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (Sender, Receiver) = mpsc::sync_channel(2); // Fixed Capacity = 2

    thread::spawn(move || {
        Sender.send("Msg 1").unwrap();
        Sender.send("Msg 2").unwrap();
        println!("Messages Sent");
    });

    println!("Received: {}", Receiver.recv().unwrap());
}
