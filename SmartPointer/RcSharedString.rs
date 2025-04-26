use std::rc::Rc;

fn main() {
    let data = Rc::new(String::from("Hello"));

    let a = Rc::clone(&data);
    let b = Rc::clone(&data);

    println!("Count: {}" , Rc::strong_count(&data));
}
