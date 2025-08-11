fn main() {
    let value = Some(3);
    let doubled = value.map(|x| x *2);
    println!("{:?}", doubled);
}