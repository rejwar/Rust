struct Holder<'a> {
    value: &'a str,
}

fn main() {
    let data = String::from("Rust");
    let holder = Holder { value: &data };
    println!("Holder value: {}", holder.value);
}
