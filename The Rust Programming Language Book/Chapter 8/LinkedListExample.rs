use std::{collections::LinkedList, str::Lines};


fn main() {
    let mut List = LinkedList::new();
    List.push_back("Rust");
    List.push_back("is");
    List.push_back("awesome!");
    println!("List: {:?}", List);
    let mut lines = "Rust\nis\nawesome!".lines();

    
}
