fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "short";
    let s2 = "longer string";
    println!("Longest: {}", longest(s1, s2));
}
