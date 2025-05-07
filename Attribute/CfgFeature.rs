#[cfg(feature = "special")]
fn SpecialFunction() {
    println!("Special Feature Enabled!");
}

fn main() {
    #[cfg(feature = "special")]
    SpecialFunction();
}
