fn main() {
    let s = String::from("Hello world");

    let slice = &s[0..5];
    println!("Slice is {}", slice);
    println!(" Original is {}", s);
}
