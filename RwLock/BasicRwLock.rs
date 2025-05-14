use std::sync::RwLock;

fn main() {
    let SharedData = RwLock::new(42);

    {
        let ReadLock = SharedData.read().unwrap();
        println!("Read Value: {}", *ReadLock);
    }

    {
        let mut WriteLock = SharedData.write().unwrap();
        *WriteLock += 10;
    }

    println!("Updated Value: {}", *SharedData.read().unwrap());
}
