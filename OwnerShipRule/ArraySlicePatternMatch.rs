fn main() {
    let array = [1, 2, 3, 4];
    match &array[1..3] {
        [a, b] => println!("Matched: {}, {}", a, b),
        _ => println!("No match"),
    }
}
