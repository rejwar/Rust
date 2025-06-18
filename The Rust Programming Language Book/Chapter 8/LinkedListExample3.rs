// 
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back("Rust");
    list.push_front("Hello");

    for item in list.iter() {
        println!("{}", item);
    }
}
