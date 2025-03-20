fn main() {
    let mut array = [1, 2, 3, 4, 5];
    let slice = &mut array[2..4]; // Mutable slice
    slice[0] *= 10;
    println!("Modified Array: {:?}", array);
}
