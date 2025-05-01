fn main() {
    let values: [i32; 6] = [4, 8, 12, 16, 20, 24];

    let my_slice = &values[..4];
    println!("{:?}", my_slice);

    let my_slice = &values[2..4];
    println!("{:?}", my_slice);

    let my_slice = &values[..];
    println!("{:?}", my_slice);
}
