fn main() {
    let mut x = 10;
    let r1 = &mut x;
    *r1 += 1;
    println!("{}", r1);
}
