struct Book {
    name: String,
}

impl Book {
    fn ShowName(&self) {
        println!("Book name: {}", self.name);
    }
}

fn main() {
    let b = Book { name: String::from("Rust in Action") };
    b.ShowName();
}
