fn main() {
    let words = vec![1,2,3,4,6,5];

    for word in words.into_iter() {
        println!("{}", word);
    }
}
