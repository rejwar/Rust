use std::sync::Arc;
use std::thread;

fn main(){
    let num = Arc::new(5);

    let num2 = Arc::clone(&num);

    thread::spawn(move || {
        println!("From thread {}" , num2);
    });

    println!(" From main {}" , num);
}