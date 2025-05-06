trait Speak {
    fn Talk(&self);
}

struct Cat;
struct Dog;

impl Speak for Cat {
    fn Talk(&self) {
        println!("Meow!");
    }
}

impl Speak for Dog {
    fn Talk(&self) {
        println!("Woof!");
    }
}

fn MakeSound(Animal: &dyn Speak) {
    Animal.Talk();
}

fn main() {
    let Kitty = Cat;
    let Puppy = Dog;

    MakeSound(&Kitty);
    MakeSound(&Puppy);
}
