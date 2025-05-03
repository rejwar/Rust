mod math {
    pub fn Calculate() {
        println!("Calculating from math module...");
    }
}

mod stats {
    pub fn Calculate() {
        println!("Calculating from stats module...");
    }
}

use math::Calculate;
use stats::Calculate; // ❌ ERROR: Ambiguous function name!

fn main() {
    Calculate(); // ❌ Compiler confusion! Which one?
}
