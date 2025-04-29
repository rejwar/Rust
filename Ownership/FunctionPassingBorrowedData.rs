fn PrintInfo(Data: &String) {
    println!("Information: {}", Data);
}

fn main() {
    let Info: String = String::from("Rust Borrowing Example");
    PrintInfo(&Info);
}
