fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = &array[0..3];
    let sum: i32 = slice.iter().sum();
    println!("Sum of slice: {}", sum);
}
