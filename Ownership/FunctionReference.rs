fn PrintData(Data: &String) {
    println!("Data: {}", Data);
}

fn main() {
    let Text: String = String::from("Hello, Rust!");
    PrintData(&Text); // Passing reference to function
}
