struct Math;

impl Math {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("Sum: {}", Math::add(5, 10));
}
