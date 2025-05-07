#![forbid(unsafe_code)]

fn main() {
    unsafe {
        println!("This will cause a compile-time error!");
    }
}
