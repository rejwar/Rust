use core::slice;

fn main() {
    let s = String::from("Hello world");
    let slice = &[0..5];
    println!(" Slice {} Original {}", slice, s);
}
