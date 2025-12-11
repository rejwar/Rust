fn main() {
    let greater = create_greater();
    greater();
}

fn create_greater() -> impl Fn() {
    let name = String::from("Rust");

    move || println!("Hello {}", name);
}
