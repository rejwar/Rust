fn main() {
    let language = String::from("Rust");

    printName(&language);
    println!("Still Owned by main {}", language);
}

fn printName (name: &String) {
    println!(" Programming Language {}" , name);
}