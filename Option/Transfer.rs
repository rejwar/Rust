fn main() {
    let Number  = Some(5);
    let Doubled = Number.map(|x| x *2);
    println!("Transfer is {:?}", Doubled);
}
