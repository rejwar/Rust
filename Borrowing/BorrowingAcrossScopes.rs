fn main() {
    let Data: String = String::from("Borrowing Across Scopes");
    {
        let Ref: &String = &Data;
        println!("Scoped Borrowing: {}", Ref);
    }
    println!("Outside Borrowing: {}", Data);
}
