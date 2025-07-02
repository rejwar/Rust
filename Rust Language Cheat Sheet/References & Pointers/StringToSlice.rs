fn main() {
    let s = String::from("New");
    let slice : &str = &s[1..5];

    println!("slice is {}", slice);
}
