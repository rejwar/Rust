use std::cell::Cell;

fn main() {
    let MutableContainer: Cell<i32> = Cell::new(10);

    println!("Initial value :{} ", MutableContainer.get());

    MutableContainer.set(20);
    println!("Updated Value :{}", MutableContainer.get());
}
