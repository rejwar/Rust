enum Calculator {
    Add(i32, i32),
    Multiply(i32, i32),
}

impl Calculator {
    fn calculate(&self) -> i32 {
        match self {
            Calculator::Add(a, b) => a + b,
            Calculator::Multiply(a, b) => a * b,
        }
    }
}

fn main() {
    let calc = Calculator::Add(10, 20);
    println!("Result: {}", calc.calculate());
}
