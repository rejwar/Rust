fn main() {
    let Condition: bool = true;
    let Value: String = if Condition {
        String::from("True Path")
    } else {
        String::from("False Path")
    };
    println!("Value: {}", Value);
}
