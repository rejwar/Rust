fn main() {
    let stack_val = 5;

    let b1 = Box::new(stack_val);

    println!("b1 points to {}", b1);

    let b2 = b1;

    println!("b2 points to {}", b2);
}
