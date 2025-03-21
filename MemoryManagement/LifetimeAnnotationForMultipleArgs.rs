fn main() {
    let a = String::from("One");
    let b = String::from("Two");
    let result = pick_first(&a, &b);
    println!("Result: {}", result);
}

fn pick_first<'a>(x: &'a str, _: &str) -> &'a str {
    x
}
