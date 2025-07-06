fn main() {
    let s = "3.14";
    let x: f64 = s.parse().unwrap();

    println!("String {} , Float {}", s,x);
}
