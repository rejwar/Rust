fn main() {
    let mut vec = vec![1,2,3,4,5,6];
    let first = &mut vec[0];
    *first = 6;
    println!("{:?}", vec);
}
