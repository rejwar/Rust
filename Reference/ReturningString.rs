fn main() {
    let City: String = CreateCity();
    println!("{City}");
}

fn CreateCity() -> String {
    String::from("New York")
}
