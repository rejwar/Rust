use std::thread; 

fn main (){
    let handle = thread::spawn(|| {
        for i in 1..= 5 {
            println!(" the child thread is {}", i);
        }
    });

    for i in 1..=10 {
        println!("Main thread is {}" , i);
    }

    handle.join().unwrap();
}