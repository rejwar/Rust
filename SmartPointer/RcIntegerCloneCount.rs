use std::rc::Rc;

fn main() {
    let SharedData = Rc::new(42);

    println!("Referrence count : {}" , Rc::strong_count(&SharedData));

    let SharedDataClone = SharedData.clone();

    println!("New referenc count {}" , Rc::strong_count(&SharedData));
}
