fn check_option(value: Option<i32>) {
    match value {
        Some(x) => println!("Received: {}", x),
        None => println!("No value available"),
    }
}

fn main() {
    check_option(Some(10));
    check_option(None);
}
