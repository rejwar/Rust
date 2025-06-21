fn main() {
    let mut data = String::from("Rust");

    let r1 = &data;
    let r2 = & data;

    println!("{} {}", r1,r2);
}
