fn main() {
    let x = 5;

    match x {
        n @ 1..=10 => println!(" Number in ranger {}", n),
        _ => println!(" Out of range "),
    }
}
