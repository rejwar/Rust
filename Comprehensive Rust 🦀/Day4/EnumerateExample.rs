fn main() {
    let list = vec!["Apple", "Bannan", "Orange"];

    for (i , fruit) in list.iter().enumerate() {
        println!("Index is {} ,{}", i , fruit);
    }
}
