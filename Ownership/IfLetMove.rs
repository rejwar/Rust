fn main() {
    let maybe_name = Some(String::from("Rustaceans"));

    if let Some(name) = maybe_name {
        println!(" Found name: {}", name);
    }
}
