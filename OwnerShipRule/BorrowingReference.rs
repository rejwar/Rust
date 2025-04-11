fn main() {
    let Data: String = String::from("Borrow Me!");
    print_message(&Data); // Borrowing data
    println!("{}", Data); // Original owner still valid
}

fn print_message(Message: &String) {
    println!("Message: {}", Message);
}
