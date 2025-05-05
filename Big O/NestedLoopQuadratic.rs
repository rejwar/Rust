fn main() {
    let Numbers = vec![1, 2, 3, 4, 5];

    for i in &Numbers {
        for j in &Numbers {
            println!("Pair: ({}, {})", i, j); // ✅ O(n²) Nested Loop
        }
    }
}
