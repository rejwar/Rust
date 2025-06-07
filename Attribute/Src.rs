// src/math.rs
pub fn Add(Num1: i32, Num2: i32) -> i32 {
    Num1 + Num2
}

// src/lib.rs
pub mod Math {
    pub use super::math::Add; // Re-exporting the `Add` function
}

fn main() {
    // Entry point for the program
}
