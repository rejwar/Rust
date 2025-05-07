#[no_mangle]
pub extern "C" fn CustomSymbolName() {
    println!("This function name remains unchanged in compiled binaries!");
}
