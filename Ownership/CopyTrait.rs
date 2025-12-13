// FileName: CopyTrait.rs
#[derive(Clone, Copy)]
struct Point {
    x: i32,
}
fn main() {
    let p1 = Point { x: 1 };
    let p2 = p1; // Copy happened
    println!("{}", p1.x); // Valid
}
