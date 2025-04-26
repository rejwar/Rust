fn main() {
    let BoxedValue: Box<i32> = Box::new(42);
    println!("Boxed value: {}", BoxedValue);
}
