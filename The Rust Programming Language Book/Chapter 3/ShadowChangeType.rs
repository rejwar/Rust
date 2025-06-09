fn main() {
    let Value = "10"; // String
    let Value: i32 = Value.parse().unwrap(); // ✅ Shadows with a different type

    println!("Converted Value: {}", Value);
}
