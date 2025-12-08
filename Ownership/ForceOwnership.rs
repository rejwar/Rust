fn main() {
    let v = vec![1, 2, 3, 4, 56, 6];

    let c = move || println!("{:?}", v);

    c();
}
