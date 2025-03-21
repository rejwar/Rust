fn main() {
    let value: Option<i32> = Some(10);
    let result = value.and_then(|v| Some(v * 2)); // Multiply by 2 if Some
    println!("{:?}", result);
}
