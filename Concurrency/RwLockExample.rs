use std::sync::RwLock;

fn main() {
    let SharedData = RwLock::new(100);

    {
        let ReadData = SharedData.read().unwrap();
        println!("Read Value: {}", *ReadData);
    }

    {
        let mut WriteData = SharedData.write().unwrap();
        *WriteData += 50;
    }

    println!("Updated Value: {}", SharedData.read().unwrap());
}
