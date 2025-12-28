use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]

struct Node {
    next: Option<Rc<Node>>,
}

fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    let b = Rc::new(RefCell::new(Node {
        next: Some(Rc::clone(&a));
    }));

    a.borrow_mut().next = Some(Rc::clone(&b));
}
