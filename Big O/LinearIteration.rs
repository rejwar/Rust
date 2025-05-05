fn main() {
    let Numbers = vec![10, 20, 30, 40, 50];
    for Num in &Numbers {
        println!("Value: {}", Num); // ✅ O(n) Iteration
    }
}
