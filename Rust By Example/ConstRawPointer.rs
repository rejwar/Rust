use std::env::consts;

fn main() {
    let x = 5;
    let ptr: *const i32 = &x;

    unsafe {
        println!("Value {}", *ptr);
    }
}
