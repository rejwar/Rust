// Question: How can we define a struct that holds references with lifetimes?

struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

fn UseStructWithLifetime() {
    let title = String::from("Rust in Action");
    let author = String::from("Tim McNamara");

    let book = Book {
        title: &title,
        author: &author,
    };

    println!("{} by {}", book.title, book.author);
}
