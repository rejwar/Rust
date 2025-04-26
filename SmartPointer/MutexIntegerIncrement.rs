use std::sync::Mutex;

fn main() {
    let SharedData: Mutex<i32> = Mutex::new(5);
    {
        let mut LockedData = SharedData.lock().unwrap();
        *LockedData += 5;

    }

    println!("Final value: {}" , SharedData.lock().unwrap());
}
