fn main() {
    let Message: String = String::from("Hello, Rust!");
    let PrintMessage = || {
        println!("{}", Message);
    };

    PrintMessage();
}
