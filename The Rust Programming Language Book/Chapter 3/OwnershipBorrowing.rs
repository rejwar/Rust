fn takes_ownership(text: String) {
    println!("Received: {}", text);
}

fn main() {
    let MyString = String::from("Rust");
    takes_ownership(MyString); // ❌ Moves ownership
    
    // println!("{}", MyString); // Error: MyString is moved!
}
