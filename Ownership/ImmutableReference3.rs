fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("{} {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
