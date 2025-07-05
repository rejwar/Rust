fn main() {
    let mut value = 5;
    let mut_ptr: *mut i32 = &mut value;
    let const_ptr: *const i32 = mut_ptr;

    unsafe  {
        println!("Value :{}", *const_ptr);
    }
}
