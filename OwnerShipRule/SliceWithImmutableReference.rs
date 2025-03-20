fn main() {
    let array = [1, 2, 3, 4];
    let slice = &array[0..2]; // Immutable reference to a slice
    println!("Slice: {:?}", slice);
}
