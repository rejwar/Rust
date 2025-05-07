#[panic_handler]
fn CustomPanic(info: &core::panic::PanicInfo) -> ! {
    println!("Custom Panic Handler: {}", info);
    loop {}
}
