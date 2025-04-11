fn main() {
    let Items: Vec<String> = vec![String::from("Item1"), String::from("Item2")];

    for Item in &Items { // Immutable borrowing for loop
        println!("Processing: {}", Item);
    }
    println!("{:?}", Items); // Items remain usable
}
