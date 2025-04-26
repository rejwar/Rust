use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let SharedData = Rc::new(RefCell::new(100));

    let a = Rc::clone(&SharedData);
    let b = Rc::clone(&SharedData);

    *a.borrow_mut() += 50;
    println!("Value for b :{}", b.borrow());
}
