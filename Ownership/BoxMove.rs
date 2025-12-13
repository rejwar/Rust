// FileName: BoxMove.rs
fn main() {
    let b1 = Box::new(5);
    let b2 = b1; // Pointer move
                 // println!("{}", b1); // Error
}
