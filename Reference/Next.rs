fn main() {
    let mut Title = String::from("Rust Book");

    let R1 =&Title;
    let R2 = &Title;

    println!("R1 -> {} ", R1);
    println!("R2 -> {}", R2);

    Title.push_str("!");
    println!("Owner -> {}", Title);
}