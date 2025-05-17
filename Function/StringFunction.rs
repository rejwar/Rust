fn process_string (data: String) {
    println!("Processed : {}", data);
}

fn main() {
    let message = String::from("Hello Rust");
    process_string(message);
}
