fn main() {
    let s = String::from("Rust");

    let r: &str = s.as_str();

    println!(" Original is safe : {}", s);
}
