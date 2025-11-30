fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    println!("{}", r1);

    let r2 = &mut s;
    println!("{}", r2);
}
