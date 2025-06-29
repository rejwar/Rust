struct Rectangle {
    width: u32,
    heigth: u32,
} 

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }
}

fn main() {
    let rect = Rectangle { width : 10 , heigth : 20};
    println!( " Area of rectangle {}", rect.area());
}
