fn main() {
    let ParsedNumber = "100".parse::<i32>().map_err(|_| "Invalid number format".to_string());
    println!("Parsed Result: {:?}", ParsedNumber);
}
