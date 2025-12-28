fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    let boxed_slice: Box<[i32]> = v.into_boxed_slice();

    println!("{:?}", boxed_slice);
}
