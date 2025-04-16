// Main Function In PascalCase
fn Main() {
    DemonstrateBasicRawPointers();
    DemonstrateUnsafeBlockUsage();
    DemonstratePointerArithmetic();
    DemonstrateFFIUsage();
}

// 1. Basic Raw Pointer Declaration
fn DemonstrateBasicRawPointers() {
    let mut Value = 42;
    
    // Immutable raw pointer
    let RawPtrImmutable: *const i32 = &Value as *const i32;
    
    // Mutable raw pointer
    let RawPtrMutable: *mut i32 = &mut Value as *mut i32;
    
    unsafe {
        println!("Immutable Pointer Value: {}", *RawPtrImmutable);
        *RawPtrMutable = 100;
        println!("Modified Value Through Pointer: {}", *RawPtrMutable);
    }
}

// 2. Unsafe Block Necessity
fn DemonstrateUnsafeBlockUsage() {
    let Data = 10;
    let DataPtr = &Data as *const i32;
    
    // SAFETY: We know the pointer is valid
    unsafe {
        println!("Data accessed through raw pointer: {}", *DataPtr);
    }
}

// 3. Pointer Arithmetic
fn DemonstratePointerArithmetic() {
    let Array = [1, 2, 3, 4, 5];
    let ArrayPtr = Array.as_ptr();
    
    unsafe {
        println!("First Element: {}", *ArrayPtr);
        println!("Second Element: {}", *ArrayPtr.add(1));
        println!("Third Element: {}", *ArrayPtr.offset(2));
    }
}

// 4. FFI Usage Example
fn DemonstrateFFIUsage() {
    // Simulating C function
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    
    let x = -42;
    let x_ptr = &x as *const i32;
    
    unsafe {
        println!("Absolute value via FFI: {}", abs(*x_ptr));
    }
}

fn main() {
    Main();
}
