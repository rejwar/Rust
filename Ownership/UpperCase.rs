fn main() {
    let original = String::from("Hello");
    let upper = to_uppercase_owned(original);

    println!(" Uppercase {}", upper);
}

fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}
