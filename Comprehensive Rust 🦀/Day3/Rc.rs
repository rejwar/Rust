use std::rc::Rc;

fn main() {
    let a:Rc<String> = Rc::new(String::from("Rustacean"));

    let b = Rc::clone(&a);
    let c = Rc::clone(&a);


    println!(" a = {}", a);
    println!(" b = {}",b);
    println!("c = {}",c);
}
