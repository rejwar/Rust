#[inline] // ✅ Compilerকে Function Inline করতে নির্দেশ দেয়
fn Square(x: i32) -> i32 {
    x * x
}

#[allow(dead_code)] // ✅ Warning নিষ্ক্রিয় করা হয়
fn UnusedFunction() {
    println!("This function is never used");
}

fn main() {
    println!("Square of 4: {}", Square(4));
}
