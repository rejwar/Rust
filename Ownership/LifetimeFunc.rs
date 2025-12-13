fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("Long");
    let s2 = String::from("Short");
    println!("{}", longest(&s1, &s2));
}
