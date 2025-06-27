fn main() {
    let chars = "Rust";

    let letters: Vec<_> = chars.chars().collect();

    println!("Print collect {:?}", letters);
}
