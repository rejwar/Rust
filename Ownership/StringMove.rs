fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2); // This will cause a compile-time error because s1 is moved to s2
}
