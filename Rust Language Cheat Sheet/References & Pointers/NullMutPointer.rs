use std::ptr;

fn main() {
    let ptr: *mut i32 = ptr::null_mut();

    unsafe  {
        if !ptr.is_null() {
            *ptr =10;

        } else {
            println!("pointer is null ");
        }
    }
}
