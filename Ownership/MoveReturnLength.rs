fn main() {
    let s = String::from("Rust");

    let len = calculate_and_consume(s);

    println!("Length {}", len);
}

fn calculate_and_consume(s: String) -> usize {
    s.len()
}
