use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        panic!(" Thread exploded ");
    });

    match handle.join() {
        Ok(_) => println!("Success"),
        Err(_) => println!("Caught panic! Proccess is still Learning"),
    }
}
