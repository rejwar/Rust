fn main() {
    let Data = String::from("Rust");

    let PrintData = move || println!("Owned Data: {}", Data);
    PrintData();
}
