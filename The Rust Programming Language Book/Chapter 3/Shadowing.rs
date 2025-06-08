fn main() {
    let X = 5;
    let X = X + 10; // ✅ Allowed: Creates a new `X`
    
    let X = X * 2; // ✅ Allowed: Shadows previous `X`
    
    println!("Shadowed X: {}", X);
}
