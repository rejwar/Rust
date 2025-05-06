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

fn ShowGreeting<T: Greet>(Person: T) {
    Person.SayHello();
}

fn main() {
    let User1 = User;
    let Admin1 = Admin;

    ShowGreeting(User1);
    ShowGreeting(Admin1);
}
