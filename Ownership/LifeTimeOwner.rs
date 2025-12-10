fn main() {
    let x = String::from("Long");
    let y = String::from("Short");

    let result = longest(x.as_str(), y.as_str());
    println!("Longest a{}", result);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
