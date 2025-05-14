use std::rc::Rc;

#[derive(Debug)]
struct Node {
    Value: i32,
    Next: Option<Rc<Node>>,
}

fn main() {
    let Node1 = Rc::new(Node { Value: 10, Next: None });
    let Node2 = Rc::new(Node { Value: 20, Next: Some(Rc::clone(&Node1)) });

    println!("{:?}", Node2);
}
