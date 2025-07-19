struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn Area(&self) -> u32 {
        self.width * self.height
    }
        fn New(width: u32 , height: u32) -> Rectangle {
    Rectangle { width, height }
}
}



fn main() {
    let rect = Rectangle::New(10,20);
    println!("Area {}", rect.Area());
}