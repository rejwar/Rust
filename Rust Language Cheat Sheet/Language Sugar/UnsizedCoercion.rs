fn print_trait(obl: &dyn std::fmt::Display) {
    println!("{}", obj);
}

fn main() {
    let s = String::from("Rust");
    print_trait(&s);
}
