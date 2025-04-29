fn GiveOwnership() -> String {
    String::from("Ownership")
}

fn TakesAndReturnOwnership(name: String) -> String {
    name
}

fn ReturnOwnership() {
    let name1 = GiveOwnership();
    let name2 = String::from("Rust");
    let name3 = TakesAndReturnOwnership(name2);


    println!("Name1 {} , Name3 {}",name1, name3);
}

fn main() {
    ReturnOwnership();
}
