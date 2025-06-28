mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
    pub fn sub(a: i32, b: i32) -> i32 { a - b }
}

use math::{self, add}; // brings both `math` and `add` into scope

fn main() {
    println!("{}", add(2, 3));     // 5
    println!("{}", math::sub(5, 2)); // 3
}
