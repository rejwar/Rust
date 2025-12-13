fn main() {
    let data = 10;

    let r1 = &data;
    let r2 = &data;
    let r3 = &data;

    println!("{} {} {}", r1, r3, r2);
}
