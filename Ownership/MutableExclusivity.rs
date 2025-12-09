fn main() {
    let mut x = 10;
    let r1 = &mut x;

    *r1 += 5;
    println!(" THe value is {}", r1);
}
