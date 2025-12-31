use std::thread;

fn main() {
    let handle = thread::Builder::new()
        .name("Data Processor ".into())
        .spawn(|| {
            let current = thread::current();
            println!("Thread Id : {:?}", current.id());
            println!("THread name: {:?}", current.name());
        })
        .unwrap();

    handle.join().unwrap();
}
