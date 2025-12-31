use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("New thread is working ");

        panic!(" Oh no ! Thread has crushed ");

        #[allow(unreachable_code)]
        "Return successful"
    });

    match handle.join() {
        Ok(msg) => {
            println!(" Thread ran Successfullty {}", msg);
        }

        Err(e) => {
            println!("We got panic in the thread ");

            if let Some(msg) = e.downcast_ref::<&str>() {
                println!("Panic message {}", msg);
            } else {
                println!(" Unknow panic happened");
            }
        }
    }

    println!(" Main thread : I am still alive &  I am still working");
}
