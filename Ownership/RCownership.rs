use std::rc::Rc;

fn main() {
    let shared_data = create_shared();
    println!("Count {}", Rc::strong_count(&shared_data));
}

fn create_shared() -> Rc<String> {
    let s = Rc::new(String::from("Shared"));
    let clone = Rc::clone(&S);
}
