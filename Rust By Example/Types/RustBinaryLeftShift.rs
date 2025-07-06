fn main() {
    let x: u8 =0b1101100;
    let y = x as u16;
    let z = y << 8;
    println!("x : {:08b}, y: {:016b} , z {:016b}",x,y,z);
}
