// How to use struct update syntax?

struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn UseStructUpdateSyntax() {
    let book1 = Book {
        title: String::from("Rust Book"),
        author: String::from("Steve"),
        pages: 300,
    };

    let book2 = Book {
        title: String::from("Advanced Rust"),
        ..book1 // copies remaining fields
    };

    println!("Book 2 Title: {}, Author: {}, Pages: {}", book2.title, book2.author, book2.pages);
}
