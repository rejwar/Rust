fn main() {
    let mut t = (1, 2);

    let a = &mut t.0;
    let b = &mut t.1;

    *a = 10;
    *b = 20;

    println!(" The value of a is {}", a);
    println!(" THe value of b is {}", b);
}
