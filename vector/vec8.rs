fn main() {
    let mut Numbers = vec![1,2,3,4,5,6,7];
    Numbers.sort();
    println!("Sorted {:?}", Numbers);

    Numbers.reverse();
    println!("{:?}", Numbers);
}