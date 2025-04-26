use std::cell::RefCell;
use std::collections::btree_map::Values;
use std::rc::Rc;

fn main() {
    let SharedData = Rc::new(RefCell::new(vec![1,2,3]));

    let Data1 = Rc::clone(&SharedData);
    let Data2 = Rc::clone(&SharedData);

    Data1.borrow_mut().push(4);
    Data2.borrow_mut().push(5);

    println!("After modification : {:?}", *SharedData.borrow());

    let Values = Rc::new(RefCell::new(5));

    let A = Rc::new(ConsCell {
        Values: Rc::clone(&Values),
        Next: RefCell::new(None),
    });

    let B = Rc::new (ConsCell{
        Values: Rc::new(RefCell::new(10)),
        Next: RefCell::new(Some(Rc::clone(&A)))
    });

    *Values.borrow_mut() += 10;
    println!("Updated value: {}" , *Values.borrow());

}

struct ConsCell {
    Values: Rc<RefCell<i32>>,
    Next: RefCell<Option <Rc<ConsCell>>>,
}
