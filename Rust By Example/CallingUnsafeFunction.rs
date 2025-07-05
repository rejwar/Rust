unsafe fn dangerous() {
    println!("Inside unsafe function");
}

fn main() {
    unsafe {
        dangerous();
    }
}
