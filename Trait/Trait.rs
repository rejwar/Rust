trait Moew {
    fn meow(&self);
}

struct Cat;

impl Moew for Cat {
    fn meow(&self) {
        println!("Meow meow");
    }
}

fn main() {}
