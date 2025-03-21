fn main() {
    let x = String::from("Hello");
    let result = show_message(&x);
    println!("{}", result);
}

fn show_message<'a>(message: &'a str) -> &'a str {
    message
}
