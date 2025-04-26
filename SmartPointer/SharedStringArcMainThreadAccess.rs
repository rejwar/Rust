use std::sync::Arc;
use std::thread::{self, Thread};

fn main() {
    let ShareData: Arc<String> = Arc::new(String::from("Concurrency"));

    let ThreadOne = {
        let CloneData = Arc::clone(&ShareData);
        thread::spawn(move || {
            println!("Thread One: {}", CloneData);
        }) 
    };

    let ThreadTwo = {
        let CloneData  = Arc::clone(&ShareData);
        thread::spawn(move || {
            println!("Thread Two: {}", CloneData);
        })
    };


    ThreadOne.join().unwrap();
    ThreadTwo.join().unwrap();
    println!("Main Thread: {}", ShareData);
    println!("Main Thread: {}", ShareData);

   //
   //
   
}
