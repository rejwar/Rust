// FileName: StringBytes.rs
fn main() {
    let s = String::from("ABC");
    let bytes = s.into_bytes(); // Consumes s
    println!("{:?}", bytes);
}
