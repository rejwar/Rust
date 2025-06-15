struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn Area(&self) -> u32 {
        self.width * self.height
    }

    fn CanHold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let r1 = Rectangle { width: 30, height: 50 };
    let r2 = Rectangle { width: 10, height: 40 };

    println!("Area: {}", r1.Area());
    println!("Can Hold: {}", r1.CanHold(&r2));
}
