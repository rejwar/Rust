fn main() {
    let mut value =99;
    let ptr: *mut i32 = &mut value;

    unsafe  {
        *ptr += 1;
        println!("Updated {}", *ptr);
    }
}
