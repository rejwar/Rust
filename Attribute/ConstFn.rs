#[inline(always)]
pub const fn Add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    const RESULT: i32 = Add(5, 10);
    println!("Result: {}", RESULT);
}
