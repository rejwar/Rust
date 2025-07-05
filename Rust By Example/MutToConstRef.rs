fn main() {
    let mut value = 10;
    let r: &i32 = &mut value;

    println!("Value {}", r);
}
