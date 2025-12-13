fn main() {
    let mut data = 10;

    let r1 = &mut data;

    *r1 += 1;
}
