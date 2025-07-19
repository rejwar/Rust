trait Speak {
    fn Speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn Speak(&self) {
        println!("Woof");
    }
}

impl Speak for Cat {
    fn Speak(&self) {
        println!("Meow");
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    dog.Speak();
    cat.Speak();
}