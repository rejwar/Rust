fn GetReference<'a> (Data: &'a String) -> &'a String {
    Data
}

fn main() {
    let Text: String = String ::from("Rust Programmer");
    let RefText: &String = GetReference(&Text);
    println!("{}", RefText);
}
