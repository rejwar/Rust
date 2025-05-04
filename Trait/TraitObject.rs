trait Speakable {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speakable for Dog {
    fn speak(&self) {
        println!("Dog says woof");
    }
}

impl Speakable for Cat {  // 'cat' থেকে 'Cat' করা হয়েছে
    fn speak(&self) {
        println!("Cat says meow");
    }
}

fn make_sound(animal: &dyn Speakable) {
    animal.speak();
}

fn main() {
    let pet1 = Dog;
    let pet2 = Cat;

    make_sound(&pet1);
    make_sound(&pet2);
}
