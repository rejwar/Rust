fn main() {
    let mut x = 10;

    let r1 = &x;
    println!("Read {}", r1);

    let r2 = &mut x;
    *r2 += 5;

    println!("Mutated {}", r2);
}
