fn main() {
    let x = String::from("Apple");
    let y = String::from("Banana");

    let result = longest(&x, &y);
    println!("Longest {}", result);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
