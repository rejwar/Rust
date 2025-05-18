fn main() {
    let mut number = 10;
    let r1 = &number;
    let r2 = &mut number;

    println!("{} {}", r1,r2);
}
