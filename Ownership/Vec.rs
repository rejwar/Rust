// FileName: VecDrain.rs
fn main() {
    let mut v = vec![String::from("A"), String::from("B")];
    let drained: Vec<_> = v.drain(..).collect(); // Moves all elements
    println!("Drained: {:?}", drained);
    println!("Original empty: {:?}", v);
}
