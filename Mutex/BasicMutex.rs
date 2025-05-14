use std::sync::Mutex;

fn main() {
    let SharedData = Mutex::new(10);

    {
        let mut Lock = SharedData.lock().unwrap();
        *Lock += 5;
    }

    println!("Updated Value: {:?}", SharedData.lock().unwrap());
}
