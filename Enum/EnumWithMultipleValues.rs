enum Operation {
    Add,
    Subtract,
    Multiply,
}

fn main() {
    let operation = Operation::Multiply;
    match operation {
        Operation::Add => println!("Adding"),
        Operation::Subtract => println!("Subtracting"),
        Operation::Multiply => println!("Multiplying"),
    }
}
