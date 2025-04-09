fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b) // টাপল রিটার্ন
}

fn main() {
    let (sum, product) = calculate(4, 5);
    println!("Sum: {}, Product: {}", sum, product);
}
