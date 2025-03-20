fn main() {
    const ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &ARRAY[..3];
    println!("Static Slice: {:?}", slice);
}
