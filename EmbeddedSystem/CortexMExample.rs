use cortex_m::asm;

fn main() {
    unsafe {
        asm::nop(); // ✅ No-Operation Assembly Instruction
    }
}
