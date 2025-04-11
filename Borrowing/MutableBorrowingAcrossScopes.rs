fn main() {
    let mut Data: String = String::from("Mutable Borrowing Across Scopes");
    {
        let Ref: &mut String = &mut Data;
        Ref.push_str(" Modified Inside Scope");
    }
    println!("{}", Data);
}
