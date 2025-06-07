fn main() {
    let option: Option<i32> = None;

    // Option::unwrap_or_default ব্যবহার করে ডিফল্ট মান পাওয়া
    let value = option.unwrap_or_default(); // 0 (i32 এর ডিফল্ট মান)
    println!("Unwrapped Value: {}", value);
}
