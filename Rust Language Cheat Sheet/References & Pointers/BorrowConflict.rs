fn main() {
    let mut data = 100;

    let r1 = &data;
    let r2 = &data;

    println!("{} {}", r1,r2);
}
