trait Animal  {
    fn sound(&self);
}

struct Dog {}
impl Animal for Dog {
    fn sound (&self) {
        println!("Woof");
    }
}

fn main() {
    let dog = Dog {};
    let animal = &dog as &dyn Animal;
    animal.sound();
}