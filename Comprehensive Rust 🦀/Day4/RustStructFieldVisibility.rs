mod data {
    pub struct Book {
        pub title : String,
        author : String,
    }

    impl Book {
        pub fn new(title: &str , author: &str) -> Self {
            Self {
                title : title.to_string(),
                author : author.to_string(), 
            }
        }
    }
}
fn main() {
    let b = data::Book::new("Rust", "MD");
    println!(" {}", b.title);
}
