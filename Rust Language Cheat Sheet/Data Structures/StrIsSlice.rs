fn ShowStringInfo (s: &str) {
    println!("Length : {} , content {}", s.len(),s);
}

fn main() {
    let text = "Hello Rust";
    ShowStringInfo(text);
}
