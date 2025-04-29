struct Book<'a> {
    Title: &'a str,
    Author: &'a str,
}

fn main() {
    let BookInfo: Book = Book {
        Title: "Rust Programming",
        Author: "Steve Klabnik",
    };

    println!("Book: {} by {}", BookInfo.Title, BookInfo.Author);
}
