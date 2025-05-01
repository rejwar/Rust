fn main() {
    let mut MyArray: [i32 ; 5] = [10,15,20,25,30];
    let MySlice: &mut [i32 ] = &mut MyArray[2..4];
    println!("MySlice L {:?}", MySlice);
}
