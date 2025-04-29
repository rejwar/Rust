use std::rc::Rc;

fn main() {
    let SharedData: Rc<String> = Rc::new(String::from("Rust Shared"));
    let RefOne: Rc<String> = Rc::clone(&SharedData);
    let RefTwo: Rc<String> = Rc::clone(&SharedData);

    println!("Reference One: {}", RefOne);
    println!("Reference Two: {}", RefTwo);
}
