// FileName: CowOwned.rs
use std::borrow::Cow;
fn main() {
    let c: Cow<str> = Cow::Owned(String::from("Owned"));
    println!("{}", c);
}
