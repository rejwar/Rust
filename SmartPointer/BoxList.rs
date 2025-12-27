enum List {
    Node(i32, Box<List>),
    Nil,
}

use List::{Nil, Node};

fn UseBoxList() {
    let list = Node(1, Box::new(Node(2, Box::new(Node(3, Box::new(Nil))))));
    println!("Linked List created is succesfully ")
}

fn main() {
    UseBoxList();
}
