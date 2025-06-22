fn main() {
    let mut stack: Vec<i32> = vec![1,2,3,4];

    while let Some(Value) = stack.pop() {
        println!("Popped {}", Value);
    }

    println!("Stack is now empty");
}
