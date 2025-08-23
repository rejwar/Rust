fn main() {
    let Title:String = String::from("Rust Book");

    let R1:&String = &Title;
    let R2:&String = &Title;

    Title.push_str("!");

    println!("R1 -> {}", R1);
    println!("R2 -> {}", R2);
}