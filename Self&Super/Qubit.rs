struct Pair(i32, i32);

impl Pair {
    fn sum(&self) -> i32 {
        self.0 + self.1
    }
}

fn main() {
    let p = Pair(4, 6);
    println!("{}", p.sum());
}
