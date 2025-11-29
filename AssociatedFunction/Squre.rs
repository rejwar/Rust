#![allow(non_snake_case)]

struct MathUtils;

impl MathUtils {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn square(n: i32) -> i32 {
        n * n
    }
}

fn main() {
    println!("Sum: {}", MathUtils::add(5, 7));
    println!("Square: {}", MathUtils::square(4));
}
