fn main() {}

fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}
