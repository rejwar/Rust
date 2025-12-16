trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Dog says: woof");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!(" Cat says: Meow");
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    for a in animals {
        a.speak();
    }
}
