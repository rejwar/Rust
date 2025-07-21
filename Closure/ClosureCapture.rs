fn main() {
    let Factory = 2;
    let Multiply = |x: i32 | x * Factory;

    println!("Scaled {}", Multiply(10));
    
}