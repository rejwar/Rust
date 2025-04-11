fn Main() {
    // Signed Integers
    let SmallSigned: i8 = -128;
    let DefaultSigned: i32 = -2_147_483_648;
    
    // Unsigned Integers
    let SmallUnsigned: u8 = 255;
    let LargeUnsigned: u64 = 18_446_744_073_709_551_615;
    
    // Integer Operations
    let Sum: i32 = DefaultSigned + 2_147_483_647;
    let Product: u64 = LargeUnsigned / 2;
    
    println!("Signed Integers: {}, {}", SmallSigned, DefaultSigned);
    println!("Unsigned Integers: {}, {}", SmallUnsigned, LargeUnsigned);
    println!("Sum: {}, Product: {}", Sum, Product);
    
    // Integer Methods
    println!("Max i32: {}", i32::MAX);
    println!("Min u8: {}", u8::MIN);
}

fn main() {
    Main();
}
