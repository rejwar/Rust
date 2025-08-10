fn main() {
    let a: Option<&str> = Some("Hello");
    let b: Option<&str> = None;

    println!("{}", a.unwrap());
    println!("{}",b.unwrap_or("world"));

}