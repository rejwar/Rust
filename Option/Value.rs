fn main() {
    let Value:Option<i32> = Some(10);
    println!("Unwrapped: {}", Value.unwrap()); // Panics if `None`
}
