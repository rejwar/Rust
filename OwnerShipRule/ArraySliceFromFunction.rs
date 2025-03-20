fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = get_slice(&array);
    println!("Slice: {:?}", slice);
}

fn get_slice(input: &[i32]) -> &[i32] {
    &input[2..] // Returns [3, 4, 5]
}
