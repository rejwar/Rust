fn main() {
    let mut value = 42;

    let ptr_const: *const i32 = &value;
    let ptr_mut: *mut i32 = &mut value;

    unsafe  {
        println!("Immutable raw {}", *ptr_const);
        *ptr_mut +=1;
        println!("Mutable raw {}", *ptr_mut);
        
    }
}
