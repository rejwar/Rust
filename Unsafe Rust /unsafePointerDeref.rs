fn main() {
    let x = 42;
    let ptr: *const i32 = &x;

    unsafe {
        println!("Value at ptr = {}", *ptr);
    }

    let bad: *const i32 = 0xdeadbeef as *const i32;
    unsafe  {
        
    }
}