struct Person {
    name: String,
    age: u8,
}

fn ShowPersonDetails(p: &Person) {
    println!("Name :{} , Age {}", p.name , p.age);
}

fn main() {
    let p1 = Person {
        name: String::from("Alice "),
        age:30,
    
    };
    ShowPersonDetails(&p1);
}
