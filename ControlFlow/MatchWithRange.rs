fn main() {
    let number = 7;
    match number {
        1..=5 => println!("Between 1 and 5"),
        6..=10 => println!("Between 6 and 10"),
        _ => println!("Other"),
    }
}
