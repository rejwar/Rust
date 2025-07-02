fn main() {
    let v = vec![1,2,3,4,5,6];

    let boxed_slice: Box<[i32]> = v.into_boxed_slice();

    println!("The BOxed slice {:?}", boxed_slice);
}
