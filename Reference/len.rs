fn CalculateLength(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hello");
    let len = CalculateLength(&s);
    println!(" {} length is {}", s , len);
}