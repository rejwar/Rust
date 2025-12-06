fn main() {
    let s1 = GivesOwnership();
    println!("{}", s1);
}

fn GivesOwnership() -> String {
    let SomeString = String::from("I am coding from a function");
    SomeString
}
