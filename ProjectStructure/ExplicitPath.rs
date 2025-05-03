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

fn main() {
    math::Calculate(); // ✅ Clear path usage
    stats::Calculate(); // ✅ No ambiguity
}
