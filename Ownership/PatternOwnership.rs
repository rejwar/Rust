fn main() {
    let my_option = Some(String::from("Hello"));

    match my_option {
        Some(s) => println!("Found {}", s),
        None => println!("Nothing"),
    }

    //println!("Original {:?}", my_option);
}
