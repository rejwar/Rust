struct Shape {
    width: u32,
    height: u32,
}

impl Shape {
    // Factory function: creates a square
    fn square(size: u32) -> Shape {
        Shape {
            width: size,
            height: size,
        }
    }

    // Factory function: creates a rectangle
    fn rectangle(w: u32, h: u32) -> Shape {
        Shape {
            width: w,
            height: h,
        }
    }
}

fn main() {
    let s = Shape::square(10); // square বানালো
    let r = Shape::rectangle(20, 30); // rectangle বানালো
}
