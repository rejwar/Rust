fn main() {
    let option = Some(5);

    if let Some(value) = option {
        println!("Values is {}", value);
    } else {
        println!("No value found ");
    }
}
