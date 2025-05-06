fn ChangeName(Name: &mut String) {
    Name.push_str(" Programming");
}

fn main() {
    let mut Name = String::from("Rust");
    ChangeName(&mut Name); // ✅ Mutable Borrowing applied

    println!("Updated Name: {}", Name);
}
