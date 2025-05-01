

struct  Rectangle {
    Width: u32 ,
    Height: u32,
}

impl Rectangle {
    fn Area(&self) -> u32 {
        self.Width * self.Height
    }
}

fn main() {
    let MyRectangle: Rectangle = Rectangle { Width: 10, Height: 5 };
    println!("Area {}" , MyRectangle.Area());
}
