enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn execute(&self, a: i32, b: i32) -> i32 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}

fn main() {
    let op = Operation::Add;
    let result = op.execute(10, 5);
    println!("Result: {}", result);
}
