use std::cell::{Ref, RefCell};

fn main() {
    let cell = create_cell();

    *cell.borrow_mut() += 50;

    println!("Value {:?}", cell);
}

fn create_cell() -> RefCell<i32> {
    RefCell::new(10)
}
