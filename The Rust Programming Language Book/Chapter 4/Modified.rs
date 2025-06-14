fn change_string(s: &mut String) {
    s.push_str(" World!");
}

fn main() {
    let mut s = String::from("Hello");
    change_string(&mut s); // Mutable borrow
    println!("{}", s);     // ✅ Modified
}
