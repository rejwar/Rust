fn ChangeString(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut MyString = String::from("Hello");
    ChangeString(&mut MyString);
    println!("Update String {}", MyString);
}
