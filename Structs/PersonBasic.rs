#[derive(Debug)]
struct Person{
    name: String,
    age : u8,
}

fn main() {
    let p = Person { name: "Arif".to_string() , age: 28};
    println!("{:?}", p);
}