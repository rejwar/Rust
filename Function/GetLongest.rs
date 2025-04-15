use std::result;

fn GetLongest<'a> (a: &'a str , b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main () {
    let s1 = "Hello";
    let s2 = "World";
    let result = GetLongest(s1, s2);
    println!("Longest: {}" , result);
}
