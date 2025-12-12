fn main() {
    let mut s = create_string();
    s.push_str("World");

    println!("{}", s);
}

fn create_string() -> String {
    String::from("Hello");
}
