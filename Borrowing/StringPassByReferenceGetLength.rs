fn PrintLength(name: &String) {
    let length = name.len();
    println!("Length is {}",length);
}

fn main() {
    let name = String::from("Alice");
    PrintLength(&name);
}
