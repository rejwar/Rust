fn main() {
    let mut Outcome: Result<u32, &str> = Err("Error");
    Outcome = Ok(100);
    println!("Outcome: {:?}", Outcome);
}
