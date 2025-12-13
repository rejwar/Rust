fn main() {
    let s1 = String::from("Small");
    let s2 = String::from("Longer String");

    println!("Longest {}", longest(&s1, &s2));
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
