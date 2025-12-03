fn TakesOwnership(s: String) {
    println!("{}", s)
}

fn main() {
    let s1 = String::from("Hello");
    TakesOwnership(s1);
}
