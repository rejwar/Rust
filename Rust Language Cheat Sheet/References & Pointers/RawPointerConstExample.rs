fn main() {
    let value: i32 = 42;

    let ptr: *const i32 = &value as *const i32;
    
    unsafe {
        println!("Value vai raw pointer {}", *ptr);
    }
}
