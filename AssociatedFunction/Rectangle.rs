// 1. Define the struct
struct Rectangle {
    width: u32,
    height: u32,
}

// 2. Define the associated function within an `impl` block
impl Rectangle {
    // This is an associated function because it does *not*
    // take `&self`, `&mut self`, or `self` as its first parameter.
    pub fn new(width: u32, height: u32) -> Rectangle {
        // We can use the type name (Rectangle) as the return type.
        Rectangle { width, height }
    }

    // A common variant is an associated function that creates a special
    // instance, like a square.
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // For comparison, this is a **method** because it takes `&self`
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // 3. Call the associated function using the **double colon (::)** syntax
    let rect1 = **Rectangle::new * *(30, 50);

    // Call another associated function
    let square = **Rectangle::square * *(25);

    // Call a method on an instance
    println!("Rectangle area: {}", rect1.area());
    println!("Square area: {}", square.area());
}
