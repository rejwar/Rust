fn main() {
    assert!(0.1+0.2 == 0.3, "Floating point addition failed");
    println!("Floating point addition is correct!");    
    assert!(0.1f32 + 0.2f32 == 0.3f32, "Floating point addition with f32 failed");
    println!("Floating point addition with f32 is correct!");
    assert!(0.1f64 + 0.2f64 == 0.3f64, "Floating point addition with f64 failed");
    println!("Floating point addition with f64 is correct!");
}
