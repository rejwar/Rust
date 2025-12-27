fn UserBoxDeref() {
    let x = Box::new(7);
    let y = *x;
    println!("Value of y {}", y);
}

fn main() {
    UserBoxDeref();
}
