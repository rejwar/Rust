fn main() {
    let mut x = 42;
    let y = &mut x;

    *y += 10;

    println!("x {} , y {}", x ,y);
}
