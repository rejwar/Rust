
use std::cell::RefCell;

fn main() {
    let MutableData: RefCell<String> = RefCell::new(String::from("Mutable data"));
    {
        let MutRefrence  = MutableData.borrow_mut();
        println!("Inside Scope : {}" , MutRefrence);

    }

    println!("Outside Scope :{}" , MutableData.borrow());
}
