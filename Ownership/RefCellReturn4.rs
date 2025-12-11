use std::cell::RefCell;

fn main() {
    let cell = get_cell();

    *cell.borrow_mut() += 10;
    println!(" Value {:?}", cell);
}

fn get_cell() -> RefCell<i32> {
    RefCell::new(5);
}
