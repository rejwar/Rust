fn main() {
    let val = Some(String::from("value"));

    match val {
        Some(V) => println!("Moved {}", V),
        None => (),
    }
}
