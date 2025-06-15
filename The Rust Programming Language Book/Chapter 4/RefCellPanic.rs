use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    let _b1 = data.borrow();       // immutable borrow
    let _b2 = data.borrow_mut();   // ❌ mutable borrow causes panic here

    println!("Value: {}", _b2);
}
