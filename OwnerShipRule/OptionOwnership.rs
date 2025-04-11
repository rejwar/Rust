fn main() {
    let Data: Option<String> = Some(String::from("Optional Data"));
    if let Some(Value) = Data {
        println!("{}", Value); // Ownership taken inside the block
    }
}
