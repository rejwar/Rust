use std::rc::Rc;

fn main() {
    let Original = Rc::new(String::from("Shared "));
    let new_owner = get_shared_copy(&Original);

    println!("Count {}", Rc::strong_count(&Original));
}

fn get_shared_copy(r: &Rc<String>) -> Rc<String> {
    Rc::clone(r)
}
