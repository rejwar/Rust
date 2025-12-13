struct Rect {
    width: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * 10
    }
}

fn main() {
    let r = Rect { width: 5 };
    println!("{}", r.area());
}
