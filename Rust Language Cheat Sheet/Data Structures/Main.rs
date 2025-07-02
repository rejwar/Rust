fn main() {
    let arr = [10, 20, 30, 40, 50];
    let b = 3;

    let slice = &arr[..=b]; // Includes elements from index 0 to 3
    println!("{:?}", slice);
}
