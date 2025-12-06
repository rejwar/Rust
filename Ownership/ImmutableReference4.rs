fn main() {
    let s = String::from("LOLO");

    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    println!("{} {} {}", r1, r2, r3);
}
