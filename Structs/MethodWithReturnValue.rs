struct Square {
    side: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

fn main() {
    let square = Square { side: 5 };
    println!("Area: {}", square.area());
}
