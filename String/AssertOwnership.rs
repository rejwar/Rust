fn process_string(data: String) {
    println!("Processing string: {}", data);
}
fn process_str(data: &str) {
    println!("Borrowed &str: {}", data);
}

fn main() {
    let Owned = String::from("Rust");
    let Borrowed = "Rust";

    process_string(Owned);
    process_str(Borrowed);
}
