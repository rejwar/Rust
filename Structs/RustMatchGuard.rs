fn main() {
    let input = Some(String::from("Hewllo"));

    match input  {
        Some(ref s ) if s == "exit" => println!("Exiting...."),
        _=> println!("Input is ignored "),
    }
}