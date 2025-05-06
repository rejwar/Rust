 trait Greet {
    fn SayHello(&self);
}

struct User;

impl Greet for User {
    fn SayHello(&self) {
        println!("Hello, Rustacean!");
    }
}

fn main() {
    let Person = User;
    Person.SayHello();
}
