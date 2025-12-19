struct User<'a> {
    name: &'a str,
    age: u32,
}

impl<'a> User<'a> {
    fn New(name: &'a str, age: u32) -> Self {
        Self { name, age }
    }

    fn print(&self) {
        println!("Name: {} , Age {}", self.name, self.age);
    }
}

fn main() {
    let name_string = String::from("Rifat");
    let user = User::New(&name_string, 22);
    user.print();

    println!("OriginaL Name: {}", name_string);
}
