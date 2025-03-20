fn main() {
    let MyArray = [1, 2, 3, 4];
    let MySlice: &[i32] = &MyArray[0..2];
    println!("MySlice: {:?}", MySlice);
}
