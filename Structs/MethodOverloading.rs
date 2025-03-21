struct Calculator;

impl Calculator {
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    fn multiply_floats(a: f64, b: f64) -> f64 {
        a * b
    }
}

fn main() {
    println!("Int Multiply: {}", Calculator::multiply(3, 4));
    println!("Float Multiply: {}", Calculator::multiply_floats(3.2, 4.1));
}
