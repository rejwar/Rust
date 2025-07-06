fn main() {
    let x = "42"; 
    println!("Original x {}",x );

    let x: i32 = x.parse().unwrap();
    println!("After shadowing , x {}, type: i32", x );
}
