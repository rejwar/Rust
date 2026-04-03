fn main() {
    let pair = (0, 5);

    match pair {
        (0..=5, y) => println!(" {}", y),
        _ => {}
    }
}
