fn main() {
    let s = String::from("Borrow me");

    calculate_lens(s);

    println!("Still valid {}", s.clone());
}

fn calculate_lens(s: String) -> usize {
    s.len()
}
