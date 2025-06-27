use std::rc::Rc;

#[derive(Debug)]
enum List {
    Node(i32, Rc<List>),
    Nil,
}

use List::{Node, Nil};

fn main() {
    let tail = Rc::new(Node(10, Rc::new(Nil)));

    let list_a = Rc::new(Node(5, Rc::clone(&tail)));
    let list_b = Rc::new(Node(3, Rc::clone(&tail)));

    println!("List A: {:?}", list_a);
    println!("List B: {:?}", list_b);
    println!("Tail ref count: {}", Rc::strong_count(&tail));
}
