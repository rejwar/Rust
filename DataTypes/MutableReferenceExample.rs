fn main() {
    let mut Value = 10;
    {
        let MyMutableReference: &mut i32 = &mut Value;
        *MyMutableReference += 20;
    }
    println!("Value: {}", Value);
}
