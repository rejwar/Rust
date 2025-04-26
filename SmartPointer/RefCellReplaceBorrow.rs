use std::{cell::RefCell, collections::btree_map::Values};

fn main() {
    let MutableData = RefCell::new(42);

    MutableData.replace(100);

    println!("New value : {}", *MutableData.borrow());

    let Value = MutableData.borrow();
    println!("Borrow's value :{}",*Value);
}
