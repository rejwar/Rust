fn main() {
    let X = 5;
    let X = X + 10; // ✅ Allowed: Creates a new instance of `X`
    let X = X * 2; // ✅ Shadows previous `X`

    println!("Shadowed X: {}", X); // Output: 30
}
