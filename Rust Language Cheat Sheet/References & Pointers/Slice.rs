fn main() {
    let arr= [10,20,30,40];
    let slice: &[i32] = &arr[1..4];

    println!("Slice {:?}", slice);
}
