fn CreateString() -> String {
    String::from("Rust String")
}

fn main() {
    let Text: String = CreateString();
    let Reference: &String = &Text;
    println!("{}", Reference);
}
