struct User {
    name: String,
    age: u32,
    active: bool,
}

impl User {
    fn New(name: String, age: u32) -> Self {
        Self {
            name,
            age,
            active: true,
        }
    }

    fn IsAdult(&self) -> bool {
        self.age >= 18
    }

    fn Deactivate(&mut self) {
        self.active = false;
    }

    fn IntoName(self) -> String {
        self.name
    }

    fn Guest() -> Self {
        Self {
            name: String::from("Guest"),
            age: 0,
            active: false,
        }
    }
}

fn main() {
    let mut user1 = User::New(String::from("Rifat"), 22);
    println!("Is Adult {}", user1.IsAdult());

    user1.Deactivate();
    println!("User Deactivate");

    let name = user1.IntoName();
    println!("Extracted Name{}", name);

    let guest = User::Guest();
    println!("Guest active {}", guest.active);
}
