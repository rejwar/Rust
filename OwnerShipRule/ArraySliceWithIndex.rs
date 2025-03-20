fn main() {
    let array = [1, 2, 3, 4];
    let slice = &array[1..];
    println!("First value in slice: {}", slice[0]);
}
