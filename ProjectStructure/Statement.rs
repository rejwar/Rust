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

use math::Calculate as MathCalc;
use stats::Calculate as StatsCalc;

fn main() {
    MathCalc(); // ✅ Calls math::Calculate()
    StatsCalc(); // ✅ Calls stats::Calculate()
}
