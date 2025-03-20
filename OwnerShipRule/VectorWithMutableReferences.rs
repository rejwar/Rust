fn main() {
    let mut vec = vec![10, 20, 30];
    {
        let ref_mut = &mut vec[1];
        *ref_mut += 10;
    }
    println!("{:?}", vec);
}
