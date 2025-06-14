fn GiveOwnership() -> String {
    String::from("Owned by caller")
}

fn main() {
    let Gift = GiveOwnership();
    println!("{}", Gift);
}
