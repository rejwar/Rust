#![feature(asm)]

fn inline_asm_example() {
    unsafe {
        asm!("mov rax, 5");
    }
}

fn main() {
    inline_asm_example();
}
