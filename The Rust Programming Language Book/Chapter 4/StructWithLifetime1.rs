struct Book<'a> {
    title: &'a str,
}

fn PrintBook(book: &Book) {
    println!("Book title: {}", book.title);
}

fn main() {
    let title = String::from("Rust Mastery");
    let b = Book { title: &title };
    PrintBook(&b);
}
