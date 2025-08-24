fn main() {
    let mut value = 5;
    let RefValue: i32 = &mut value;
    *RefValue += 10;
    println!("New value is {}", RefValue);
}