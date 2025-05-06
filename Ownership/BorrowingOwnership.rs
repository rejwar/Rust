fn PrintName(Name: &String) {
    println!("Name: {}", Name);
}

fn main() {
    let Name = String::from("Rust");
    PrintName(&Name); // ✅ Borrowing applied

    println!("Original Name: {}", Name); // ✅ Valid
}
