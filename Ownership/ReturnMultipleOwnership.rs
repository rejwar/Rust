use std::rc::Rc;
fn main() {
    let shared = create_shared();
    println!("Count {}", Rc::strong_count(&shared));

    fn create_shared() -> Rc<String> {
        let original = Rc::new(String::from("Data"));
        Rc::clone(&original)
    }
}
