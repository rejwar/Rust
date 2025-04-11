fn main() {
    let MutData: String = String::from("Hello, Mutable Borrowing!");
    let RefMutData: &mut String = &mut MutData; // Mutable borrowing
    RefMutData.push_str(" Modified!");
    println!("Updated Data: {}", RefMutData);
}
