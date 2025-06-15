
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("Hello");
    print_length(&s); // s is borrowed, not moved
    println!("{}", s); // s is still valid
}
