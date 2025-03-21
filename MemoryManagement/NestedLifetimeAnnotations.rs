fn main() {
    let x = String::from("Hello");
    let y = String::from("World");
    let result = concatenate_with_lifetime(&x, &y);
    println!("{}", result);
}

fn concatenate_with_lifetime<'a, 'b>(a: &'a str, b: &'b str) -> &'a str {
    a
}
