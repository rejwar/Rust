fn main() {
    let ValidData = Some("Rust").unwrap();
    let InvalidData: Option<&str> = None;

    println!("{}" , InvalidData.unwrap());
}
