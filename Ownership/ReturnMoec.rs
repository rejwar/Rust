// FileName: ReturnOwnedMod.rs
fn create() -> String { String::from("Hello") }
fn main() {
    let mut s = create();
    s.push_str(" World");
}