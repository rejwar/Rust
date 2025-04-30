fn AppendMessage(Message: &mut String) {
    Message.push_str("-Updated");
}

fn main() {
    let mut Text:String = String::from("Hello Rust");
    AppendMessage(&mut Text);
    println!("{}", Text);
}
