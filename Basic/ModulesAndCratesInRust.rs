// In a file named `math.rs`
pub mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

// In the main file
mod math;

fn main() {
    let result = math::add(5, 10);
    println!("5 + 10 = {}", result);
}
