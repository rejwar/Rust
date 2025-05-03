mod math {
    pub fn Square(N: i32) -> i32 {
        N * N
    }
}

fn main() {
    use math::Square;
    println!("Square {}" , Square(5));
}
