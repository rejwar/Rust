fn main(){
use std::cell::RefCell;


let x = RefCell::new(5);

*x.borrow_mut () += 1; 

}
