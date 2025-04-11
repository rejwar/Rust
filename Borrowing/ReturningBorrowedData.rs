fn BorrowData<'a>(Data: &'a String) -> &'a String {
    Data
}

fn main() {
    let Info: String = String::from("Return Borrowed Example");
    let Ref: &String = BorrowData(&Info);
    println!("{}", Ref);
}
