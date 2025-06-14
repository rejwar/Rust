fn give_ownership() -> String {
    String::from("From function")
}

fn take_and_return(s: String) -> String {
    s
}

fn main() {
    let s1 = give_ownership();      // Ownership received
    let s2 = take_and_return(s1);   // Ownership passed and returned
    println!("{}", s2);             // ✅ OK
}
