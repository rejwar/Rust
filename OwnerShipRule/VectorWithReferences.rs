fn main() {
    let mut vec = vec![1, 2, 3];
    let ref1 = &vec[0]; // Immutable reference
    let ref2 = &vec[1];
    println!("ref1: {}, ref2: {}", ref1, ref2);
}
