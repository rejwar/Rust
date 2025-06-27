use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Counter {
    count: Rc<RefCell<i32>>,
}

fn main() {
    let shared = Rc::new(RefCell::new(0));

    let a = Counter { count: Rc::clone(&shared) };
    let b = Counter { count: Rc::clone(&shared) };

    *a.count.borrow_mut() += 1;
    *b.count.borrow_mut() += 2;

    println!("Shared count: {}", shared.borrow()); // 3
}
