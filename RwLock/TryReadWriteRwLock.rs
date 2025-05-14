use std::sync::RwLock;

fn main() {
    let SharedData = RwLock::new(200);

    if let Ok(ReadLock) = SharedData.try_read() {
        println!("Non-blocking Read: {}", *ReadLock);
    }

    if let Ok(mut WriteLock) = SharedData.try_write() {
        *WriteLock += 20;
        println!("Non-blocking Write Completed!");
    }
}
