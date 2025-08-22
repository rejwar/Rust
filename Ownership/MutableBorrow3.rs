fn main() {
    let mut Text: String = String::from("Mutable Borrow");
    BorrowMutable(&mut Text);
    println!("Owner After modification is {}", Text);
}

fn BorrowMutable(Input: &mut String) {
    Input.push_str("Updated");
    println!("Modified Value is {}", Input);
}