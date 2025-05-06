fn main() {
    let Name = String::from("Rust");

    let Ref1 = &Name;
    let Ref2 = &Name;

    println!("Reference 1: {}", Ref1);
    println!("Reference 2: {}", Ref2);
}
