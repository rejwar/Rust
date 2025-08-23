// Kaizen: Step 6 — Return a reference that points to an argument with longer lifetime.

fn main() {
    let Title = String::from("Rust Book");
    let View = Longest(&Title, &String::from("Guide"));
    println!("View -> {}", View);
    println!("Owner -> {}", Title);
}

fn Longest(A: &String, B: &String) -> String {
    if A.len() >= B.len() { A.to_string() } else { B.to_string() }
}
