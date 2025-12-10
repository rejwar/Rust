fn main() {
    let (s, n) = get_pair();
    println!("String {} int {} ", s, n);
}

fn get_pair() -> (String, i32) {
    (String::from("Rust"), 2025);
}
