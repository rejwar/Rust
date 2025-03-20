fn main() {
    let Numbers: [i32; 3] = [10, 20, 30];
    let Slice = &Numbers[0..2];
    println!("Slice: {:?}", Slice);
}
