fn main() {
    let mut s = String::from("cargo\n");
    let r1 = &mut s;
    r1.push_str("Run");

    println!("{}", r1);
}
