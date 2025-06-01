fn main() {
    let OwnedString = String::from("Rust");
    let Borrowedstr: &str = &OwnedString;

    let NewString = Borrowedstr.to_string();
}
