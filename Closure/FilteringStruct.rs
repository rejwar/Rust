#[derive(Debug)]

struct Person   {
    name: String,
    age : u32,
}

fn main() {
    let people = vec![
        Person {name: String::from("Alice") , age : 25},
        Person { name: String::from("Bob") , age: 17},
        Person {name : String::from("Charlie"),age: 30},
    ];

    let adults: Vec<_> = people.into_iter().filter(|p| p.age >= 18).collect();

    println!("Adults {:.?}", adults);
}