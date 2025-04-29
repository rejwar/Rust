fn CompareStrings<'a>(StrOne: &'a str, StrTwo: &'a str) -> &'a str {
    if StrOne.len() > StrTwo.len() {
        StrOne
    } else {
        StrTwo
    }
}

fn main() {
    let First: &str = "Hello";
    let Second: &str = "Rust Programming";
    let LongerString: &str = CompareStrings(First, Second);
    println!("Longer String: {}", LongerString);
}
