fn DisplayMessage(Message: &String) {
    println!("Message: {}", Message);
}

fn main() {
    let Data: String = String::from("Immutable Borrowing Example");
    DisplayMessage(&Data);
}
