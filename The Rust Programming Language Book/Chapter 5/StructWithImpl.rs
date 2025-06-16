// How to use `impl` block with struct methods?

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method that returns area
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that returns true if the rectangle is square
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn UseRectangleStruct() {
    let rect = Rectangle { width: 10, height: 10 };
    println!("Area: {}", rect.area());
    println!("Is square? {}", rect.is_square());
}

fn main() {
    UseRectangleStruct();
}
