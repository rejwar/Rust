struct Person<'a> {
    Name: &'a str,
}

fn main() {
    let UserName: String = String::from("Alice");
    let User: Person = Person { Name: &UserName };

    println!("User Name: {}", User.Name);
}
