fn main() {
    let x = 10;
    let ptr = &x;

    println!("x = {}", x);
    println!("ptr points to {}", *ptr);
}