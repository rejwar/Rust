fn main() {
    let s = String::from("Keep Ownership");

    calculate_len(&s);

    println!("Still valid {}", s);
}

fn calculate_len(r: &String) -> usize {
    r.len()
}
