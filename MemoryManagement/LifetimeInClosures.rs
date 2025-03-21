fn main() {
    let text = String::from("Closure with lifetime");
    let closure = |x: &str| x.len();
    println!("Length: {}", closure(&text));
}
