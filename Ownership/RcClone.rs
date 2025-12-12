use std::rc::Rc;

fn main() {
    let original = Rc::new(String::from("Shared"));
    let clone1 = Rc::clone(&original);

    println!("Count {}", Rc::strong_count(&original));
}
