fn main() {
    let name = String::from("Rust");

    print_name(&name);
    println!("{}", name);
}

fn print_name(name: &String) {
    println!("{}", name);
}
