fn main() {
    let mut stack = Vec::from([1,2,3,4,5]);

    while let Some(top) = stack.pop() {
        println!("Popped {}", top);
    }
}
