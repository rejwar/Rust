fn main() {
    let pointerSizeUnsignedInteger: usize = 42; // আকার সিস্টেম আর্কিটেচারের উপর নির্ভরশীল
    println!("Pointer Size Unsigned Integer: {}", PointerSizeUnsignedInteger);

    let maxUsize: usize = usize::MAX;
    println!("Maximum Usize Value: {}", MaxUsize);

    let minUsize: usize = usize::MIN; // Unsigned এর সর্বনিম্ন মান সবসময় 0
    println!("Minimum Usize Value: {}", MinUsize);

    // একটি পয়েন্টারকে usize-এ কাস্ট করা (unsafe ব্লক ব্যবহার করতে হয়)
    let some_value: i32 = 10;
    let pointer_to_value: *const i32 = &some_value;

    unsafe {
        let pointerAsUsize: usize = pointer_to_value as usize;
        println!("Pointer as usize: {}", PointerAsUsize);
    }
}
