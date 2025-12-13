// FileName: CowModify.rs
use std::borrow::Cow;
fn main() {
    let mut c: Cow<str> = Cow::Borrowed("Hello");
    c.to_mut().push_str(" World"); // Becomes Owned here
    println!("{}", c);
}
