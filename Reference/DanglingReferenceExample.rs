fn GetString() -> String {
    String::from("Rust Programmer ")
}

fn main() {
    let Data:String = GetString();
    let Reference: &String = &Data;

    println!("{}", Reference);
}
