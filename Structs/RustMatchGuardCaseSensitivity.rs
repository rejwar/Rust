fn main() {
    let input = Some(String::from("Exit"));

    match input {
        Some(ref s) if s == "Exit" => println!("Exit.."),
        _=> println!(" The input is ignored "),
    }
}