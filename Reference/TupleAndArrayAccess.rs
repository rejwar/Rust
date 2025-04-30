fn main() {
    let Registration: [bool; 3] = (true , false , true);
    let first:bool = Registration.0;
    println!("{first} and {Registration:?}");

    let languages: [String;2 ] = (String::from("Rust") , String::from("JavaScript"));
     let first: &String = &languages.0;
     println!("{first } and {languages:?}");
}
