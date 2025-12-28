use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("Hello"));
    let b = Rc::clone(&a);

    println!("{}", a);
    println!(" {}", b);
}
