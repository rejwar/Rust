struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rect = Rectangle::new(10, 20);
    println!("Width: {}, Height: {}", rect.width, rect.height);
}
