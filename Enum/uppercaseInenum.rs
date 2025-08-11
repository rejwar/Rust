fn main() {
    let name = Some("rifat");
    let y = name.map(|n|n.to_uppercase());
    println!("{:?}", y);
}