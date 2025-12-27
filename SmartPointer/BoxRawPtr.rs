fn BoxRawPointer() {
    let b = Box::new(123);
    let raw = Box::into_raw(b);
    println!("Raw Pointer address = {:?}", raw);

    unsafe {
        let restored = Box::from_raw(raw);
        println!("Restored value = {}", restored);
    }
}

fn main() {
    BoxRawPointer();
}
