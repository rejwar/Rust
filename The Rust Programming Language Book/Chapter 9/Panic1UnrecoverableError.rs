// Question: How do you trigger and understand an unrecoverable error using panic! in Rust?

fn CausePanicExample() {
    println!("Program started...");

    panic!("Unexpected condition occurred! Shutting down.");

    // This line will never be reached
    println!("Program ended.");
}
