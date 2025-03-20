fn main() {
    let array = [10, 20, 30, 40, 50];
    let slice = &array[1..4]; // Partial slice
    println!("Slice: {:?}", slice);
}
