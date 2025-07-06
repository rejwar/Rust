fn main() {
    let s = "42";
    let x = s.parse::<i32>().unwrap();
    println!("String : {} , Integer {}", s,x); 
}
