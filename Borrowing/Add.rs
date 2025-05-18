fn AddSuffix(name: &mut String) {
    name.push_str(".rs");
}

fn main() {
    let mut name = String::from("Rustacean");
    AddSuffix(&mut name);
    println!("{name}");
}
