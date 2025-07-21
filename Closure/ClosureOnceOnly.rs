fn main() {
    let text = String::from("Rust");
    let consume = move || println!("Consumed {}", text);


    consume();
}