fn CheckRange(x:i32){
    match x {
        0..=10 => println!("x is in the range 0 to 10"),
        11..=20 => println!("x is in the range 11 to 20"),
        _ => println!("x is out of range"),
        
    }
}
    fn main() {
        CheckRange(5);

    }
