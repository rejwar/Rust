#![allow(non_snake_case)]

trait Summarizable {
    fn author() -> String; // associated function in trait
}

struct Book {
    title: String,
}

impl Summarizable for Book {
    fn author() -> String {
        String::from("Rabindranath Tagore")
    }
}

fn main() {
    // Trait এর associated function কল করা হচ্ছে
    println!("Author: {}", Book::author());
}
