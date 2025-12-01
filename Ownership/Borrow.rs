fn main() {
    let name = String::from("Rust");

    PrintName(&name);
    println!("{}", name)
}

fn PrintName(NameRef: &String) {
    println!("{}", NameRef);
}
