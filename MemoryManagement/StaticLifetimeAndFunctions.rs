fn main() {
    println!("{}", static_string());
}

fn static_string() -> &'static str {
    "I live forever!"
}
