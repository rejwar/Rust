fn main() {
    let message = String::from("Rust is amazing!");
    print_message(&message);
}

fn print_message(message: &str) {
    println!("{}", message);
}
