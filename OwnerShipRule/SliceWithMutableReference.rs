fn main() {
    let mut numbers = [1, 2, 3, 4];
    let slice = &mut numbers[1..3]; // Mutable reference to a slice
    slice[0] = 10;
    println!("Modified Array: {:?}", numbers);
}
