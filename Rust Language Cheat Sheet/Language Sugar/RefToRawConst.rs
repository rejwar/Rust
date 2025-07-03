fn main() {
    let value = 42;
    let ptr: *const i32 = &value;

    unsafe  {
        println!("Rawvalue is {}", *ptr);
        
    }
}
