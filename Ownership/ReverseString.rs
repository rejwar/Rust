fn main() {
    let original = String::from("Rust");
    let reversed = reversed_it(original);

    println!(" Reveersed String {}", reversed);
}

fn reversed_it(s: String) -> String {
    s.chars().rev().collect()
}
