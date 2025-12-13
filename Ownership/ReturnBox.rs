// FileName: ReturnBox.rs
fn get_box() -> Box<i32> {
    Box::new(10)
}
fn main() {
    let b = get_box();
    println!("{}", *b);
}
