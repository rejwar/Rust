fn main() {
    let Title = String::from("Rust Book");

    let R1:&String = &Title;

    println!("The main owner {}", Title);
    println!(" Line A -> {}", R1);

    let R2:&String = &Title;
    println!("Line c -> {}", R2);

}