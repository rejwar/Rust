#![feature(llvm_asm)]

pub fn fast_add(a: i32, b: i32) -> i32 {
    let result: i32;
    unsafe {
        llvm_asm!("add $1, $2, $0" : "=r"(result) : "r"(a), "r"(b));
    }
    result
}

fn main() {
    println!("LLVM Optimized Add: {}", fast_add(3, 5));
}
