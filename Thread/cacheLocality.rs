use std::thread;

fn main() {
    let data = vec![1; 1_000_000];
    let chunk_size = data.len() / 2;

    let h1 = thread::spawn(move || {});

    let h2 = thread::spawn(move || {});

    h1.join().unwrap();
    h2.join().unwrap();
}
