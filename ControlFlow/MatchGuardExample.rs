fn main() {
    let number = 10;
    match number {
        n if n > 0 => println!("Positive"),
        _ => println!("Not positive"),
    }
}
