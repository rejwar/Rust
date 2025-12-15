#[allow(dead_code)]

trait Greet {
    fn say_hello(&self) -> String;
}

struct User {
    name: String,
    age: u8,
    email: String,
}

impl Greet for User {
    fn say_hello(&self) -> String {
        format!(
            "Hello , my name is {} and I am {} years old",
            self.name, self.age
        )
    }
}

fn greet_everyone<T: Greet>(item: &T) {
    println!("SomeOne is greeting {}", item.say_hello());
}

fn main() {
    let john = User {
        name: String::from("John Doe"),
        age: 39,
        email: String::from("John@gmail.com"),
    };

    println!("--- Direct Trait Method call---");
    println!("{}", john.say_hello());

    println!("\n ---- Generics function call --- ");
    greet_everyone(&john);
}
