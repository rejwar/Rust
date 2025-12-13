// FileName: CowBorrowed.rs
use std::borrow::Cow;
fn main() {
    let s = "Static";
    let c: Cow<str> = Cow::Borrowed(s);
    println!("{}", c);
}
