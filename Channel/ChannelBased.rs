use std::sync::mpsc;
use std::thread;

#[derive(Debug)]

enum State {
    Init,
    Running,
    Done,
}

fn main() {
    let (tx , rx ) = mpsc::channel();

    thread::spawn(move || {
        tx.send(State::Init).unwrap();
        tx.send(State::Running).unwrap();
        tx.send(State::Done).unwrap();

    });

    for state in rx.iter() {
        println!("Transitioned to {:?}" , state);
    }
}