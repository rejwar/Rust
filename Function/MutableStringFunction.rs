fn append_text(data: &mut String) {
    data.push_str("Is awesome");
}

fn main() {
    let mut text = String::from("Rust ");
    append_text(&mut text);
    println!("{}", text);
}
