fn main() {
    let factor = 2;

    let multiply = |x: i32 | -> i32 {x * factor };

    let result = multiply(10);

    println!("Result {}", result);
}