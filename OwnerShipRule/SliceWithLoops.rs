fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = &array[..3];
    for value in slice {
        println!("{}", value);
    }
}
