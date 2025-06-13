fn main() {
    let Bytes = vec![82,90,91,92,75];
    let word = String::from_utf8(Bytes).expect("Invalid UFt-8");
    println!("{}", word);
}
