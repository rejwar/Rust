fn GetLongerString<'a>(Str1: &'a str, Str2: &'a str) -> &'a str {
    if Str1.len() > Str2.len() {
        Str1
    } else {
        Str2
    }
}

fn main() {
    let First: &str = "Short";
    let Second: &str = "Much Longer String";
    let Result: &str = GetLongerString(First, Second);

    println!("Longer String: {}", Result);
}
