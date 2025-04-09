fn multiply(a: i32, b: i32) -> i32 {
    a * b // শেষ এক্সপ্রেশন রিটার্ন হয় (সেমিকোলন নেই!)
}

fn main() {
    let result = multiply(3, 4);
    println!("Result: {}", result); // আউটপুট: 12
}
