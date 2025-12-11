fn main() {
    let msg = get_static_msg();
    println!("{}", msg);
}

fn get_static_msg() -> &'static str {
    "I live forever in Binary"
}
