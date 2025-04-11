fn main() {
    let Items: Vec<String> = vec![String::from("Item1"), String::from("Item2")];

    for Item in Items {
        println!("Processing: {}", Item);
        // Ownership of `Item` ends after iteration
    }
    // println!("{:?}", Items); // Error! Ownership lost after loop
}
