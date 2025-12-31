use std::thread;

fn main() {
    let builder = thread::Builder::new().stack_size(8 * 1024);

    let handle = builder
        .spawn(|| {
            println!("Small stack thread running !");
        })
        .unwrap();

    handle.join().unwrap();
}
