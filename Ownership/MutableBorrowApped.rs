fn main() {
    let mut s = String::from("Hello");
    add_rust(&mut s);
    println!("Modified {}", s);
}

fn add_rust(s: &mut String) {
    s.push_str("Rust");
}
