fn main() {
    let Value = Some(5);
    match Value {
        Some(v) => println!("Value: {}", v),
        None => println!("No value"),
    }
}
