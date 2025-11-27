struct Book {
    title: String,
    pages: u32,
    current_page: u32,
}

impl Book {
    fn new(title: String, pages: u32) -> Self {
        Self {
            title,
            pages,
            current_page: 0,
        }
    }

    fn read(&mut self, pages: u32) {
        self.current_page += pages;
        if self.current_page > self.pages {
            self.current_page = self.pages;
        }
    }

    fn progress(&self) -> f64 {
        if self.pages == 0 {
            0.0
        } else {
            self.current_page as f64 / self.pages as f64 * 100.0
        }
    }

    fn is_finished(&self) -> bool {
        self.current_page >= self.pages
    }
}

fn main() {
    let mut b = Book::new("Rust Book".to_string(), 100);

    b.read(30);
    println!("Progress = {}%", b.progress());

    b.read(80);
    println!(
        "Progress = {}%, finished? {}",
        b.progress(),
        b.is_finished()
    );
}
