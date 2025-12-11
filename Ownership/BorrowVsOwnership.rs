fn main() {
    let s1 = String::from("Owner");
    let len = CalculateLen(&s1);

    println!("{} is still here . Length {}", s1, len);
}

fn CalculateLen(s: &String) -> usize {
    s.len()
}
