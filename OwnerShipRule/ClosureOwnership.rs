fn main() {
    let Data: String = String::from("Closure Example");
    let Closure = move || {
        println!("Data: {}", Data); // Ownership transferred to closure
    };
    Closure();
}
