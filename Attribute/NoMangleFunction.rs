#[no_mangle]
pub extern "C" fn PublicFunction() {
    println!("This function will not be renamed by the compiler!");
}
