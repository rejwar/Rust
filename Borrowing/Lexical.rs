fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;

    println!("{} and {} ", r1,r2);

    let r3 = &mut s;
    println!("{}", r3);
}
