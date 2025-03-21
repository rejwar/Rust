fn main() {
    let value: Option<i32> = Some(20);
    let result = value.map_or(0, |v| v * 2); // Multiply by 2 or use default
    println!("{}", result);
}
