#[naked]
pub extern "C" fn naked_function() {
    unsafe {
        asm!("ret"); // Immediate return
    }
}
