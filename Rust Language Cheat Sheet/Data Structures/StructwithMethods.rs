struct  Rectangle {
    width: i32,
    height : i32,
}

impl  Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 40,
    };

    println!( " Area :  {}" , rect.area());
}

