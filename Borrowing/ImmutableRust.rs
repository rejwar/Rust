fn PrintMessage(name: &String) {
    println!("Hello {}", name);
}

fn main() {
    let name = String::from("Rust");
    PrintMessage(&name);
    println!("Main function {}",name);
}
