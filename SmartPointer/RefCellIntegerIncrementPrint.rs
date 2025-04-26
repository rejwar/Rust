use std::cell::RefCell;

fn main() {
    let x = RefCell::new(10);

    *x.borrow_mut() +=5;

    println!("Value:{}" , x.borrow());
}
