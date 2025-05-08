use std::sync::Mutex;

fn main() {
    let SharedData = Mutex::new(42);
    
    {
        let mut Data = SharedData.lock().unwrap();
        *Data += 1;
    }

    println!("Updated Value: {}", SharedData.lock().unwrap());
}
