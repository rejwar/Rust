use std::thread;
use std::time::Duration;


fn PrintMessage(id: u32){
    println!("Thread{} strated", id);

    thread:: sleep(Duration::from_secs(2));
    println!("Thread{} finished.", id);

}

fn main() {
    let handles: Vec<_> = (13)
    .map(|id| thread::spawn(move || PrintMessage(id)))
    .collect();
}
