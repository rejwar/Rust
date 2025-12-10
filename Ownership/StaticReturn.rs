fn main() {
    let msg = get_greeting();
    println!("{}", msg);
}

fn get_greeting() -> &'static str {
    "hello world"
}
