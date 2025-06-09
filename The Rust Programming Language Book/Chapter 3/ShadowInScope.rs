fn main() {
    let X = 5;

    {
        let X = X * 2; // ✅ Shadows `X` inside inner scope
        println!("Inner Scope X: {}", X);
    }

    println!("Outer Scope X: {}", X);
}
