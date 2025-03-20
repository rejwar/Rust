fn main() {
    let numbers = vec![1, 2, 3];
    for num in &numbers { // Immutable reference in loop
        println!("{}", num);
    }
}
