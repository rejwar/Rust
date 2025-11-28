trait Greet {
    fn hello(&self) -> String;
}

struct Person;

impl Greet for Person {
    fn hello(&self) -> String {
        String::from("Hello!")
    }
}

fn main() {
    let p = Person;
    println!("{}", p.hello());
}
