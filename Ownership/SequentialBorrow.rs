fn main() {
    let r1 = &x;
    println!("Read {}", r1);

    let r2 = &mut x;
    *r2 += 5;
    println!("Updated {}", r2);
}
