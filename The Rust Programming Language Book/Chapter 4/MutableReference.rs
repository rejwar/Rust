fn append_world(text: &mut String) {
    text.push_str(" World");
}

fn main() {
    let mut greeting = String::from("Hello");
    append_world(&mut greeting);
    println!("{}", greeting);
}
