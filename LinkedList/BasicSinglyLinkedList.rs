use std::option::Option;

struct Node {
    Data: i32,
    Next: Option<Box<Node>>,
}

fn main() {
    let Node1 = Node { Data: 10, Next: None };
    let Node2 = Node { Data: 20, Next: Some(Box::new(Node1)) };

    println!("First Node Data: {}", Node2.Data);
}
