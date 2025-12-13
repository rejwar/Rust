fn main() {
    let mut x = 10;
    {
        let r1 = &mut x;
        *r1 += 1;
    }

    let r2 = &mut x;
    *r2 += 2;

    println!("{}", x);
}
