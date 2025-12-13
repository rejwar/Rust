use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let data = Rc::new(RefCell::new(String::from("Start")));
    *data.borrow_mut() = String::from("Modified");
    println!("{:?}", data);
}
