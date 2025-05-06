trait Greet {
    fn SayHello(&self);
}

struct User;
struct Admin;

impl Greet for User {
    fn SayHello(&self) {
        println!("Hello from User!");
    }
}

impl Greet for Admin {
    fn SayHello(&self) {
        println!("Hello from Admin!");
    }
}

fn main() {
    let Persons: Vec<Box<dyn Greet>> = vec![Box::new(User), Box::new(Admin)];

    for Person in Persons.iter() {
        Person.SayHello();
    }
}
