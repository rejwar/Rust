#![feature(target_feature)]
#![cfg_attr(target_arch = "x86_64", target_feature(enable = "sse2"))]

unsafe fn fast_sqrt(x: f64) -> f64 {
    x.sqrt() // ✅ Optimized for SSE2 instruction set
}

fn main() {
    unsafe {
        println!("Optimized Square Root: {}", fast_sqrt(16.0));
    }
}
