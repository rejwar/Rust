use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("hello"));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("{}", a);
    println!(" Strong count = {}", Rc::strong_count(&a));
}
