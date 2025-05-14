use std::sync::Mutex;

fn main() {
    let SharedData = Mutex::new(42);

    if let Ok(mut Lock) = SharedData.try_lock() {
        *Lock += 8;
    }

    println!("Updated Data: {}", *SharedData.lock().unwrap());
}
