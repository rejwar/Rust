fn WhileLoopExample() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LiftOFf");
}

fn main() {
    WhileLoopExample();
}
