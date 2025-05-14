use std::rc::Rc;

fn main() {
    let SharedValue = Rc::new(100);

    let Ref1 = Rc::clone(&SharedValue);
    let Ref2 = Rc::clone(&SharedValue);

    println!("Shared Data: {}", SharedValue);
    println!("Reference Count: {}", Rc::strong_count(&SharedValue));
}
