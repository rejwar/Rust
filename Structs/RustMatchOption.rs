fn main() {
    let value = Some(42);

    match value {
        Some(x) => println!("The value is x {}" ,x),
        _=> (),
    }
}