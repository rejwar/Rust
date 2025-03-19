trait Printable {
    fn print(&self);
}

struct Book {
    title: String,
}

impl Printable for Book {
    fn print(&self) {
        println!("Book: {}", self.title);
    }
}

fn print_item<T: Printable>(item: T) {
    item.print();
}

fn main() {
    let book = Book { title: String::from("The Rust Programming Language") };
    print_item(book);
}
