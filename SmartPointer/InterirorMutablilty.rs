use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let v = Rc::new(RefCell::new(5));
    let a = Rc::clone(&v);
    let b = Rc::clone(&v);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 1;

    println!("{}", v.borrow());
}
