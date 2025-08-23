fn main() {
    let Title: String = String::from("Rust Book");

    let R1: &String = &Title;
    let R2: &String = &Title;

    println!(" The main owner -> {}", Title);
    println!("a Line is -> {}", R1);
    println!(" B line is -> {}", R2);

}