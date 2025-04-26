use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    let Leaf = Rc::new(TreeNode{
        Value: 3,
        Parent: RefCell::new(Weak::new()),
        Children: RefCell::new(vec![]),

    });

    println!("Leaf parent = {:?}" , Leaf.Parent.borrow().upgrade());

    let Branch = Rc::new(TreeNode {
        Value: 5,
        Parent: RefCell::new(Weak::new()),
        Children: RefCell::new(vec![Rc::clone(&Leaf)]),
    });

    *Leaf.Parent.borrow_mut() = Rc::downgrade(&Branch);

    println!("Leaf Parent = {:?}" , Leaf.Parent.borrow().upgrade());

    println!("Branch strong count :{}", Rc::strong_count(&Branch));
    println!("Branch strong count :{}", Rc::weak_count(&Branch));
}

#[derive(Debug)]
struct TreeNode{
    Value :i32 ,
    Parent: RefCell<Weak < TreeNode>>,
    Children: RefCell<Vec<Rc<TreeNode>>>,
}
