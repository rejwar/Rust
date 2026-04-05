use std::cell::Cell;

fn main() {
    let x = Cell::new(5);

    x.set(10);
    println!("{}", x.get());
}


