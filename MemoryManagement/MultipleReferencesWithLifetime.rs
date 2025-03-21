fn main() {
    let first = String::from("First");
    let second = String::from("Second");
    let result = longer(&first, &second);
    println!("Longer string: {}", result);
}

fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
