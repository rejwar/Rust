// FileName: StructRef.rs
struct RefHolder<'a> {
    r: &'a String,
}
fn main() {
    let s = String::from("Hi");
    let h = RefHolder { r: &s };
    println!("{}", h.r);
}
