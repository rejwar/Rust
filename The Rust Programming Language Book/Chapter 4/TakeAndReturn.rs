fn Modify(Data: String) -> String {
    Data + " modified"
}

fn main() {
    let Original = String::from("Data");
    let Updated = Modify(Original); // Ownership moved and returned
    println!("{}", Updated);
}
