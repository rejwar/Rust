struct Square {
    side: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

fn main() {
    let sq = Square { side: 4 };
    println!("Area: {}", sq.area());
}
