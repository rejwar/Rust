fn main() {
    let s = "42";
    let x = i32::from_str_radix(s, 10).unwrap();
    println!("String {} , Integer {}",s,x );
}
