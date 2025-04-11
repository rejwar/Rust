fn DisplayGeneric<T: std::fmt::Debug>(Data: &T) {
    println!("{:?}", Data);
}

fn main() {
    let Info: String = String::from("Generics Borrowing Example");
    DisplayGeneric(&Info);
}
