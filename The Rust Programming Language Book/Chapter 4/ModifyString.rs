fn main() {
    let mut Message = String::from("Hello");

    Message.push(',');
    Message.push_str("World");
    println!("{}", Message);
}
