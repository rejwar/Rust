fn main() {
    let text1 = String::from("Text1");
    let text2 = String::from("Text2");
    let result = combine(&text1, &text2);
    println!("Combined: {}", result);
}

fn combine<'a, 'b>(_x: &'a str, y: &'b str) -> &'b str {
    y
}
