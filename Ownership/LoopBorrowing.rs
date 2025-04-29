fn main() {
    let Data: Vec<String> = vec![String::from("Item1"), String::from("Item2")];

    for Item in &Data {
        println!("Processing: {}", Item);
    }
}
