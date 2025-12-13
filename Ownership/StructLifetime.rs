struct Book<'a> {
    title: &'a str,
}

fn main() {
    let text = String::from("Rust Book");
    let b = Book { title: &text };
    println!("{}", b.title);
}
