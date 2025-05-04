trait Greetable {
    fn Greet(&self) -> String {
        String::from("Hello Rustacenas")
    }
}

struct User;

impl Greetable for User {} 
fn main() 
{
    let Person = User;
    println!("{}" , Person.Greet());
}
