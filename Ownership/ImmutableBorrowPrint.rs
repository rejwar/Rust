fn main() {
    let s = String::from("Borrow");
    printBoprrowme(&s);
    println!("Still valid {}", s);
}

fn printBoprrowme(s: &String) {
    println!("Borrowed {}", s);
}
