struct  Book {
    title: String,
    pages: u32,
}

fn main() {
    let b = Book {
        title: String::from ("Rust Guide"),
        pages:250,
    };

    let Book {title,pages} = b;
    println!("{} has {} pages ", title,pages);
}
