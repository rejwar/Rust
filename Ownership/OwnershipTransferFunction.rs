fn TakesOwnership(name: String) {
    println!("Name : {}", name);
}

fn OwnershipWithFunction() {
    let name = String::from("Rust");
    TakesOwnership(name);
}

fn main() {
    OwnershipWithFunction();
}
