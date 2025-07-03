
#[repr(packed)]
struct Packed {
    a: u8,
    b: u32, // unaligned field
}

fn main() {
    let packed = Packed { a: 1, b: 42 };

    // ❌ This would be UB: let ptr = &packed.b as *const u32;
    // ✅ Safe: create raw pointer without reference
    let ptr = &raw const packed.b;

    unsafe {
        println!("Read unaligned: {}", ptr.read_unaligned());
    }
}
