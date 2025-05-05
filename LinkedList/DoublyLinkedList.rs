use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    Data: i32,
    Prev: Option<Rc<RefCell<Node>>>,
    Next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn New(Data: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { Data, Prev: None, Next: None }))
    }
}

fn main() {
    let Head = Node::New(10);
    let Tail = Node::New(20);

    Head.borrow_mut().Next = Some(Rc::clone(&Tail));
    Tail.borrow_mut().Prev = Some(Rc::clone(&Head));

    println!("Head Data: {}", Head.borrow().Data);
    println!("Tail Data: {}", Tail.borrow().Data);
}
