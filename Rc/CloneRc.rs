use std::rc::Rc;

fn main() {
    let Data = Rc::new("Rust Rc Example");

    let RefA = Rc::clone(&Data);
    let RefB = Rc::clone(&Data);

    println!("Reference Count: {}", Rc::strong_count(&Data));
}
