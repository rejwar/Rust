struct User {
    name: String,
}

impl User {
    fn Consume(self) {
        println!("Goodbye, {}", self.name);
    }
}

fn main() {
    let u = User { name: String::from("Alice") };
    u.Consume(); 
    
}
