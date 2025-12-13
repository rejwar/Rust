// FileName: VecSplit.rs
fn main() {
    let mut v = vec![String::from("A"), String::from("B")];
    let v2 = v.split_off(1); // Moves index 1 to end
    println!("v: {:?}, v2: {:?}", v, v2);
}
