fn main() {
    let Data: Option<String> = Some(String::from("Option Borrowing Example"));
    if let Some(Ref) = &Data {
        println!("Borrowed: {}", Ref);
    }
    println!("Original: {:?}", Data);
}
