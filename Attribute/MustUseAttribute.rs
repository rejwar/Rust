#[must_use]
fn ComputeValue(x: i32) -> i32 {
    x * 2
}

fn main() {
    ComputeValue(10); // ✅ Compiler Warning: Return Value not used!
}
