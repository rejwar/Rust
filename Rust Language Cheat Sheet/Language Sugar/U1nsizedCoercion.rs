fn print_slice(slice: &[i32]) {
    println!("{:?}", slice);
}

fn main() {
    let array = [1, 2, 3];
    print_slice(&array); // coercion: &[i32; 3] → &[i32]
}
