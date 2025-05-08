use cortex_m_rt::exception;

#[exception]
fn SysTick() {
    // ✅ Handle Timer Interrupt
    println!("SysTick Interrupt Triggered!");
}
