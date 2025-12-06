fn main() {
    let s2 = String::from("Hello");
    let (s2, len) = calculated_length(s2);

    println!(" THe length of{} is {}", s2, len);
}

fn calculated_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
