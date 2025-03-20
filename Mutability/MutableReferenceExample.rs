fn main() {
    let mut Value = 10;
    let MutableRef = &mut Value;
    *MutableRef += 5;
    println!("Value: {}", Value);
}
