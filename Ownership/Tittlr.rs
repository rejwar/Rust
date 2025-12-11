struct Book {
    title: String,
}

fn main() {
    let b1 = Book {
        title: String::from("Rust Book"),
    };

    let b2 = b1;

    println!(" New owner : {}", b2.title);
}
