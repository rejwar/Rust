fn main() {
    let mut numbers = vec![1, 2, 3];
    for num in &mut numbers { // Mutable reference in loop
        *num *= 2;
    }
    println!("{:?}", numbers);
}
