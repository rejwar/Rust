use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let a = Rc::new(RefCell::new(10));
    *a.borrow_mut() += 1;
    println!("{}", a.borrow());
}
