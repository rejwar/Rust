fn main() {
    let mut Title:String = String::from("Ruist book");

    let R = &Title;
    Title.push_str("2nd Edition ");
    println!(" {}", Title)
}