#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

#[panic_handler]
fn HandlePanic(_info: &PanicInfo) -> ! {
    loop {} // Handle system panic without `std`
}
