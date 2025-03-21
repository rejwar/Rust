struct Person<'a> {
    name: &'a str,
    age: u32,
}

fn main() {
    let name = "Alice";
    let person = Person { name, age: 30 };
    display_person(&person);
}

fn display_person<'a>(person: &'a Person<'a>) {
    println!("Name: {}, Age: {}", person.name, person.age);
}
