fn main() {
    let array = [10, 20, 30, 40, 50];
    print_slice(&array[1..4]); // Passing slice
}

fn print_slice(slice: &[i32]) {
    println!("Slice: {:?}", slice);
}
