fn main() {
    let mut s = String::from("Hello");
    let r = &mut s;

    r.push_str("World");
    println!("{}", r);
}
