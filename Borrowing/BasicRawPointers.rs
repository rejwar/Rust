fn main() {
    let x = 30;
    let RawPointX = &x as *const i32;

    unsafe {
        println!("Value of RawPointX (deferenced): {}", *RawPointX);
    }

    let mut y = 40;
    let MutableRawPointY = &mut y as *mut i32;

    unsafe {
        *MutableRawPointY += 10;
        println!("New value of y (through raw pointer) :{}",y);
    }
}
