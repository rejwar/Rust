fn borrow_string(s: &String) {
    println!("Borrowed: {}", s);
}

fn main() {
    let s = String::from("Rusty");
    borrow_string(&s); // Pass by reference
    println!("Still owned: {}", s); // ✅ Still usable
}
