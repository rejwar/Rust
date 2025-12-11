fn main() {
    let num = 42;
    let r = &num;

    let ptr = r as *const i32;

    unsafe {
        println!("Value via pointer {}", *ptr);
    }
}
