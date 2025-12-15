trait Fly {
    fn Fly(&self);
}

trait Swim {
    fn Swim(&self);
}

struct Duck;

impl Fly for Duck {
    fn Fly(&self) {
        println!(" The duck can fly ");
    }
}

impl Swim for Duck {
    fn Swim(&self) {
        println!("The duck can swim");
    }
}

fn main() {
    let donald = Duck;

    donald.Fly();
    donald.Swim();
}
