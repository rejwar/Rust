use std::rc::Rc;
#[derive(Debug)]

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn main() {
    let tail = Rc::new(Node {
        value: 2,
        next: None,
    });
    let head = Rc::new(Node {
        value: 1,
        next: Some(Rc::clone(&tail)),
    });
    println!("{:?}", head);
}
