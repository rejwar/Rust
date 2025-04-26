use std::fmt;

fn main() {
    let boxed_integer = Box::new(10);
    println!("Boxed value: {}", boxed_integer);

    let value = *boxed_integer;
    println!("Dereferenced value: {}", value);

    let list = ConsList::Cons(2, Box::new(ConsList::Cons(3, Box::new(ConsList::Nil))));
    println!("List: {:?}", list);

    let large_data = [0; 1000];  // No need for Box here
    println!("Large data array size: {}", large_data.len());

    let behavior: Box<dyn Behavior> = Box::new(Dog);
    behavior.make_sound();
}

#[derive(Debug)]
enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil,
}

trait Behavior {
    fn make_sound(&self);
}

struct Dog;

impl Behavior for Dog {
    fn make_sound(&self) {
        println!("Woof");
    }
}
