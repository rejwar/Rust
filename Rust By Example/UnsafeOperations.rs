fn main() {
    let x = 10;

    let ptr = &x as *const i32;

    unsafe  {
        println!("Value at ptr {}", *ptr);
    }
}
