fn main() {
    let value: Option<i32> = Some(5);
    let result = value.map(|v| v + 10); // Add 10 if Some
    println!("{:?}", result);
}
