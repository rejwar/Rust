/// Appends "IsAwesome" to the input string
fn change_name(name: &mut String) {
    name.push_str("IsAwesome");
}

fn mutable_borrow() {
    let mut name = String::from("Rust");
    change_name(&mut name);
    
    println!("Updated name: {}", name);
}

fn main() {
    mutable_borrow();
}
