fn main() {
    let s = String::from("Hello, world!");
    let r1 = &s;
    let r2 = &s;

    println!("{} {}", r1, r2);
}
