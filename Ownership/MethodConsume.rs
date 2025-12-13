struct Rect {
    width: u32,
}

impl Rect {
    fn consume(self) {
        println!("Consumed width: {}", self.width);
    }

    fn main() {
        let r = Rect { width: 5 };
        r.consume();
    }
}
