fn calculate(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}
fn main() {
    let (Sum, Product) = calculate(4, 5);
    println!("Sum: {}, Product: {}", Sum, Product);
}
