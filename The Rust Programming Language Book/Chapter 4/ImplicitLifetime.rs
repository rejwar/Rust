fn PrintRef(s: &str) {
    println!("{}",s);
}

fn main() {
    let name = "Rust";
    PrintRef(name);
}
