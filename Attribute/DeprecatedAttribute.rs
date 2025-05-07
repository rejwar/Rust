#[deprecated(since = "1.2.0", note = "Use NewFunction instead")]
fn OldFunction() {
    println!("This function is deprecated!");
}

fn main() {
    OldFunction(); // ✅ Compiler Warning: Function Deprecated!
}
