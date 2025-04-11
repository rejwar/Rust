fn DisplayMessage(Message: &String) {
    println!("Message: {}", Message);
}

fn main() {
    let Data: String = String::from("Borrowed By Function!");
    DisplayMessage(&Data);
}
