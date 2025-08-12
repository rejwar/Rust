struct Score {
    value: i32,
}

impl Score {
    fn AddOne(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut s = Score { value: 0 };
    s.AddOne();
    println!("Score: {}", s.value); // 1
}
