fn main() {
    let received = create_msg();
    println!("I got it {}", received);
}

fn create_msg() -> String {
    String::from("Message from Function ");
}
