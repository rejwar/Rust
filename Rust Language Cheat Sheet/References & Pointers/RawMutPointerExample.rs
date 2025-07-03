fn main() {
    let mut value = 42;
    let ptr: *mut i32 = &mut value;

    unsafe  {
        *ptr +=1;
        println!("updated value is {}", *ptr);
    }
}
