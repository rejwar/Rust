fn main() {
    let num = 42;

    let r: &i32 = &num;
    let ptr: *const i32 = r as *const i32;

    unsafe {
        println!(" Pointer value {}", *ptr);
    }
}
