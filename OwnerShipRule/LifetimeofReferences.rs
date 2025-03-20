fn example<'a>(input: &'a str) -> &'a str {
    input
}

fn main() {
    let s = String::from("Lifetime Example");
    let result = example(&s);
    println!("{}", result);
}
