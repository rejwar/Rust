use std::thread;

fn main() {
    let num = 30;

    let handle = thread::spawn(move || {
        println!("New theread counting on");
        num * 2
    });

    println!("Main THread is waiting for the result");

    match handle.join() {
        Ok(result) => println!("We got th result from the thread {}", result),
        Err(_) => println!("There is a error in this thread"),
    }
}
