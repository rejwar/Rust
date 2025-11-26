struct Student {
    name: String,
    age: u8,
    gpa: f32,
}

fn main() {
    let s1 = Student {
        name: String::from("Rejwar"),
        age: 20,
        gpa: 3.50,
    };

    println!("Name is {}", s1.name);
    println!("Age is {}", s1.age);
    println!("gpa is {}", s1.gpa);
}
