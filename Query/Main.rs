fn main() {
    let option = Some(5);
    let filtered = option.filter(|&x| x > 3);
    println!("{:?}", filtered ); // Output: Some(5)

    let filtered_none = option.filter(|&x| x < 3);
    println!("{:?}", filtered_none); // Output: None
}
