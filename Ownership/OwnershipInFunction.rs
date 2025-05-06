fn PrintName(Name: String) {
    println!("Name: {}", Name);
}

fn main() {
    let Name = String::from("Rust");
    PrintName(Name); // ✅ Ownership moved

    // println!("{}", Name); ❌ Error! Name moved to function.
}
