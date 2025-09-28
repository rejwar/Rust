use core::task;
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
struct Task {
    id: u32,
    name: String,
}

fn main() {
    let (tx , rx) = mpsc::channel();

    thread::spawn(move || {
        let task = Task::{id: 1 , name: "compile" . to_string()};
        tx.send(task).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Received Task is {:?}", received);
}