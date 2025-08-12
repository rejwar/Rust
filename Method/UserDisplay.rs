// This program answers: "How to define a method with self to take ownership of the struct?"

struct User {
    name: String,
    age: u8,
}

impl User {
    fn ShowInfo(self) {
        // self here takes ownership of the whole struct
        println!("Name: {}, Age: {}", self.name, self.age);
        // After this method ends, the struct is dropped automatically
    }
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };

    user.ShowInfo();

    // ❌ This will be an error because ownership has moved
    // println!("{}", user.name);
}
