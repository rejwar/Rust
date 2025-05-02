fn main() {
    let Number = "42".parse::<i32>().unwrap(); // Using Turbofish
    println!("Parsed Number: {}", Number);
}
