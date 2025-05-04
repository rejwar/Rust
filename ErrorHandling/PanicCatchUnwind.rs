use std::panic;

fn main() {
    let Result = panic::catch_unwind(|| {
        panic!("Critical Failure");
    });

    match Result {
        Ok(_) => println!("No Panic occured"),
        Err(_) => println!(" Panic handled gracefully"),
    }
}
