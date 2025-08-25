struct Person { Name: String , Age: u32} 
fn main() {
    let mut PersonBox: Box<Person> = Box::new(Person { Name: String::from("Alice"), Age: 23 });
    PersonBox.Age +=5;

    println!("Name is {} Age {}", PersonBox.Age , PersonBox.Name);
}