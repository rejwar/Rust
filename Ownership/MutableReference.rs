fn main() {
    let mut Data: String = String::from("Rust"); // Mutable data
    let RefMutData: &mut String = &mut Data; // Mutable borrowing

    RefMutData.push_str(" Programming!"); // Modifying data through reference
    println!("Updated Data: {}", RefMutData);
}
