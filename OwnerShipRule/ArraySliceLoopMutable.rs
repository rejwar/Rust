fn main() {
    let mut array = [5, 10, 15, 20];
    let slice = &mut array[1..3]; // Mutable slice
    for val in slice.iter_mut() {
        *val *= 2;
    }
    println!("Modified Array: {:?}", array);
}
