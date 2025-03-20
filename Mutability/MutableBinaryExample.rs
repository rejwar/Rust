fn main() {
    let mut BinaryFlag: u8 = 0b0000_0001;
    BinaryFlag <<= 1;
    println!("BinaryFlag: {:08b}", BinaryFlag);
}
